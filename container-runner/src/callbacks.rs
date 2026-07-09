//! `EnvoyCallbacks` implementation: the extension point Rivet calls to start/stop
//! the actor and to route tunneled traffic. Here the "actor" is a child game-server
//! process; traffic is proxied to its local port.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex as TokioMutex;

use rivet_envoy_client::config::{
	BoxFuture, EnvoyCallbacks, HttpRequest, HttpResponse, WebSocketHandler, WebSocketSender,
};
use rivet_envoy_client::handle::EnvoyHandle;
use rivet_envoy_client::protocol;

use crate::child::{ChildProcess, SpawnSpec, log_prefix};
use crate::input::ActorInput;

/// Static runner configuration derived from the CLI/env.
pub struct RunnerConfig {
	/// Child command template (program + fixed args) from `-- <command...>`.
	pub command_template: Vec<String>,
	/// Default local port for the child when `input.port` is absent.
	pub default_child_port: u16,
	/// SIGTERM→SIGKILL grace period on stop.
	pub stop_grace: Duration,
	/// How long to wait for the child's port to open before failing the start.
	pub readiness_timeout: Duration,
}

/// Shared across the serverless runtime and the envoy client.
pub struct ContainerRunnerCallbacks {
	cfg: Arc<RunnerConfig>,
	/// Running children keyed by actor id. Concurrency is 1 in the Cloud Run model,
	/// but a map keeps the lifecycle explicit and race-free.
	actors: Arc<TokioMutex<HashMap<String, Arc<ChildProcess>>>>,
}

impl ContainerRunnerCallbacks {
	pub fn new(cfg: Arc<RunnerConfig>) -> Self {
		Self {
			cfg,
			actors: Arc::new(TokioMutex::new(HashMap::new())),
		}
	}

	/// Stop every running child. Called on container-runner shutdown so children are
	/// never orphaned (belt-and-suspenders alongside the child's `kill_on_drop`).
	pub async fn stop_all(&self, grace: Duration) {
		let children: Vec<Arc<ChildProcess>> =
			self.actors.lock().await.drain().map(|(_, c)| c).collect();
		if children.is_empty() {
			return;
		}
		println!("runner: shutdown, stopping {} child(ren)", children.len());
		for child in children {
			child.stop(grace).await;
		}
	}
}

impl EnvoyCallbacks for ContainerRunnerCallbacks {
	fn on_actor_start(
		&self,
		handle: EnvoyHandle,
		actor_id: String,
		generation: u32,
		config: protocol::ActorConfig,
		_preloaded_kv: Option<protocol::PreloadedKv>,
	) -> BoxFuture<anyhow::Result<()>> {
		let cfg = self.cfg.clone();
		let actors = self.actors.clone();
		Box::pin(async move {
			// One actor per container (Cloud Run 1:1 model). Guard against two failure modes
			// that otherwise collide on the fixed child port:
			//  - A duplicate start for the SAME actor (engine retry) -> idempotent no-op.
			//  - A DIFFERENT actor landing on this container -> refuse loudly with an
			//    actionable message instead of spawning a second game server that can't bind.
			{
				let guard = actors.lock().await;
				if let Some(existing) = guard.get(&actor_id) {
					if !existing.has_exited() {
						println!(
							"{} runner: actor already running, ignoring duplicate start",
							log_prefix(&actor_id, config.key.as_deref())
						);
						return Ok(());
					}
				}
				if let Some(other) = guard
					.iter()
					.find(|(id, c)| id.as_str() != actor_id.as_str() && !c.has_exited())
					.map(|(id, _)| id.clone())
				{
					anyhow::bail!(
						"container-runner hosts one actor per container (Cloud Run 1:1 model), \
                         but actor {other} is already running; refusing to start {actor_id}. \
                         Configure the serverless runner with max_concurrent_actors=1 and \
                         Cloud Run request concurrency=1."
					);
				}
			}

			let input = ActorInput::parse(config.input.as_deref())?;
			let child_port = input.port.unwrap_or(cfg.default_child_port);

			// input.command overrides the CLI template; input.args are appended.
			let mut parts = input
				.command
				.clone()
				.unwrap_or_else(|| cfg.command_template.clone());
			if parts.is_empty() {
				anyhow::bail!(
					"no child command: CLI template is empty and input.command was not provided"
				);
			}
			parts.extend(input.args.clone());
			let program = parts.remove(0);

			let spec = SpawnSpec {
				program,
				args: parts,
				env: input.env,
				child_port,
				actor_id: actor_id.clone(),
				key: config.key.clone(),
			};

			tracing::info!(
				boot_id = crate::inspector::boot_id(),
				actorId = %actor_id,
				"actor starting on this container instance"
			);

			let child = Arc::new(ChildProcess::spawn(spec, cfg.readiness_timeout).await?);

			// Watchdog: if the child exits unexpectedly (not via on_actor_stop), tell
			// the engine to stop the actor. `remove` returns `Some` only if we won the
			// race against a deliberate stop, avoiding a double report.
			{
				let child = child.clone();
				let actors = actors.clone();
				let handle = handle.clone();
				let actor_id = actor_id.clone();
				tokio::spawn(async move {
					let status = child.wait_exit().await;
					// `remove` returns Some only if we won the race against a deliberate
					// stop (on_actor_stop) or a runtime shutdown (stop_all) — i.e. this was
					// an UNEXPECTED child exit. Report it so the engine applies crash policy.
					if actors.lock().await.remove(&actor_id).is_some() {
						let prefix = log_prefix(&actor_id, child.key.as_deref());
						println!(
							"{prefix} runner: child exited unexpectedly ({status}), reporting actor stopped"
						);
						handle.stop_actor(
							actor_id,
							Some(generation),
							Some(format!("child exited: {status}")),
						);
					}
				});
			}

			// Ensure the inspector bearer token exists in KV key [0x03] so the dashboard
			if let Err(e) = crate::inspector::ensure_inspector_token(&handle, &actor_id).await {
				tracing::warn!(error = ?e, actorId = %actor_id, "failed to ensure inspector token");
			}

			actors.lock().await.insert(actor_id, child);
			Ok(())
		})
	}

	fn on_actor_stop(
		&self,
		_handle: EnvoyHandle,
		actor_id: String,
		_generation: u32,
		_reason: protocol::StopActorReason,
	) -> BoxFuture<anyhow::Result<()>> {
		let actors = self.actors.clone();
		let grace = self.cfg.stop_grace;
		Box::pin(async move {
			let child = actors.lock().await.remove(&actor_id);
			if let Some(child) = child {
				child.stop(grace).await;
			}
			Ok(())
		})
	}

	fn on_shutdown(&self) {
		// Envoy-driven shutdown hook (sync). Actual child teardown is the async
		// `stop_all`, invoked from main's shutdown path.
		tracing::info!("envoy shutdown");
	}

	fn fetch(
		&self,
		_handle: EnvoyHandle,
		actor_id: String,
		_gateway_id: protocol::GatewayId,
		_request_id: protocol::RequestId,
		request: HttpRequest,
	) -> BoxFuture<anyhow::Result<HttpResponse>> {
		let actors = self.actors.clone();
		Box::pin(async move {
			// Serve the custom inspector UI directly, BEFORE proxying to the child. Only
			// GET requests under `/inspector/ui/` are intercepted.
			if request.method.eq_ignore_ascii_case("GET")
				&& crate::inspector::is_inspector_ui_path(&request.path)
			{
				return Ok(crate::inspector::inspector_response());
			}

			// The dashboard polls `/metadata` on the actor surface. The child can't answer
			// it (WebSocket-only), so answer it here.
			if request.method.eq_ignore_ascii_case("GET")
				&& crate::inspector::is_metadata_path(&request.path)
			{
				return Ok(crate::inspector::metadata_response());
			}

			let Some(child_port) = actors.lock().await.get(&actor_id).map(|c| c.child_port) else {
				anyhow::bail!("fetch: no running child for actor {actor_id}");
			};
			crate::proxy::http_proxy(child_port, request).await
		})
	}

	fn websocket(
		&self,
		_handle: EnvoyHandle,
		actor_id: String,
		_gateway_id: protocol::GatewayId,
		_request_id: protocol::RequestId,
		_request: HttpRequest,
		path: String,
		_headers: HashMap<String, String>,
		_is_hibernatable: bool,
		_is_restoring_hibernatable: bool,
		_sender: WebSocketSender,
	) -> BoxFuture<anyhow::Result<WebSocketHandler>> {
		let actors = self.actors.clone();
		Box::pin(async move {
			let Some(child_port) = actors.lock().await.get(&actor_id).map(|c| c.child_port) else {
				anyhow::bail!("websocket: no running child for actor {actor_id}");
			};
			crate::proxy::ws_proxy(child_port, path).await
		})
	}

	fn can_hibernate(
		&self,
		_actor_id: &str,
		_gateway_id: &protocol::GatewayId,
		_request_id: &protocol::RequestId,
		_request: &HttpRequest,
	) -> BoxFuture<anyhow::Result<bool>> {
		// Game servers hold live in-memory state; never hibernate a connection.
		Box::pin(async { Ok(false) })
	}
}
