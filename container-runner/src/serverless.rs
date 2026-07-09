//! The serverless HTTP front door.
//!
//! Cloud Run routes the engine's outbound `POST /api/rivet/start` to this server;
//! that request holds the container open for the actor's lifetime (SSE response).
//! This is a slimmed re-implementation of `rivetkit-core`'s `serverless.rs` +
//! `serverless_http.rs`, wired to our own `ContainerRunnerCallbacks` (the upstream
//! `CoreServerlessRuntime` is hard-coded to its in-process actor registry).

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::{Context, Result};
use axum::Router;
use axum::body::{Body, Bytes};
use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use futures_util::StreamExt;
use serde_json::json;
use tokio::net::TcpListener;
use tokio::sync::{Mutex as TokioMutex, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::sync::CancellationToken;

use rivet_envoy_client::config::{ActorName, EnvoyConfig};
use rivet_envoy_client::envoy::start_envoy;
use rivet_envoy_client::handle::EnvoyHandle;
use rivet_envoy_client::protocol;

use crate::callbacks::ContainerRunnerCallbacks;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const SSE_PING_INTERVAL: Duration = Duration::from_secs(1);
const SSE_PING_FRAME: &[u8] = b"event: ping\ndata:\n\n";
const SSE_STOPPING_FRAME: &[u8] = b"event: stopping\ndata:\n\n";

/// Static settings for the serverless runtime.
pub struct ServerlessSettings {
	/// Runner version reported to the engine (used for draining on deploy).
	pub version: u32,
	/// Base path the engine calls (default `/api/rivet`).
	pub base_path: String,
	/// Actor names this runner advertises.
	pub actor_names: Vec<String>,
	/// Max `/start` payload size.
	pub max_start_payload_bytes: usize,
}

pub struct ServerlessRuntime {
	settings: ServerlessSettings,
	callbacks: Arc<ContainerRunnerCallbacks>,
	envoy: Arc<TokioMutex<Option<EnvoyHandle>>>,
	shutting_down: AtomicBool,
	/// Cancelled to bring the whole process down. `main` owns the other end.
	exit: CancellationToken,
}

impl ServerlessRuntime {
	pub fn new(
		settings: ServerlessSettings,
		callbacks: Arc<ContainerRunnerCallbacks>,
		exit: CancellationToken,
	) -> Arc<Self> {
		Arc::new(Self {
			settings,
			callbacks,
			envoy: Arc::new(TokioMutex::new(None)),
			shutting_down: AtomicBool::new(false),
			exit,
		})
	}

	fn prepopulate(&self) -> HashMap<String, ActorName> {
		self.settings
			.actor_names
			.iter()
			.map(|name| {
				(
					name.clone(),
					ActorName {
						metadata: json!({}),
					},
				)
			})
			.collect()
	}

	fn metadata_json(&self) -> serde_json::Value {
		let actor_names: serde_json::Map<String, serde_json::Value> = self
			.settings
			.actor_names
			.iter()
			.map(|name| (name.clone(), json!({ "metadata": {} })))
			.collect();
		// The engine's serverless metadata validation only accepts runtime == "rivetkit"
		// (engine `pegboard_serverless_metadata_fetch`). We speak the same envoy protocol,
		// so we present as "rivetkit" to be accepted.
		json!({
			"runtime": "rivetkit",
			"version": PKG_VERSION,
			"envoyProtocolVersion": protocol::PROTOCOL_VERSION,
			"actorNames": actor_names,
			"envoy": { "kind": { "serverless": {} }, "version": self.settings.version },
		})
	}

	/// Lazily start (and cache) the envoy client using our callbacks.
	async fn ensure_envoy(&self, headers: &StartHeaders) -> Result<EnvoyHandle> {
		if self.shutting_down.load(Ordering::Acquire) {
			anyhow::bail!("runtime is shutting down");
		}
		let mut guard = self.envoy.lock().await;
		if let Some(handle) = guard.as_ref() {
			return Ok(handle.clone());
		}
		let callbacks: Arc<dyn rivet_envoy_client::config::EnvoyCallbacks> = self.callbacks.clone();
		let handle = start_envoy(EnvoyConfig {
			version: self.settings.version,
			endpoint: headers.endpoint.clone(),
			token: headers.token.clone(),
			namespace: headers.namespace.clone(),
			pool_name: headers.pool_name.clone(),
			prepopulate_actor_names: self.prepopulate(),
			metadata: Some(json!({ "container-runner": { "version": PKG_VERSION } })),
			not_global: true,
			debug_latency_ms: None,
			callbacks,
		})
		.await;
		*guard = Some(handle.clone());
		Ok(handle)
	}

	/// Drain the cached envoy on shutdown.
	pub async fn shutdown(&self) {
		self.shutting_down.store(true, Ordering::Release);
		let handle = self.envoy.lock().await.take();
		if let Some(handle) = handle {
			handle.shutdown_and_wait(false).await;
		}
	}

	/// End the process when an actor's `/start` ends. This container hosts exactly one
	/// actor for its lifetime; when that actor is gone the instance must not be reused
	/// for the next one. The runner is PID 1 in the image, so exiting stops the
	/// container and the platform reaps the instance.
	///
	/// Cancelling (rather than `process::exit`) lets `main` run the existing teardown:
	/// SIGTERM the child, drain the envoy, and let axum's graceful shutdown flush the
	/// in-flight `/start` SSE response before the socket closes.
	pub fn request_exit(&self, actor_id: &str, reason: &str) {
		tracing::info!(actorId = %actor_id, reason, "actor finished, exiting container");
		self.exit.cancel();
	}
}

#[derive(Debug)]
struct StartHeaders {
	endpoint: String,
	token: Option<String>,
	pool_name: String,
	namespace: String,
}

fn header(headers: &HeaderMap, name: &str) -> Option<String> {
	headers
		.get(name)
		.and_then(|v| v.to_str().ok())
		.map(|s| s.to_string())
		.filter(|s| !s.is_empty())
}

fn parse_start_headers(headers: &HeaderMap) -> Result<StartHeaders> {
	let pool_name = header(headers, "x-rivet-pool-name")
		.or_else(|| header(headers, "x-rivet-runner-name"))
		.context("x-rivet-pool-name header is required")?;
	Ok(StartHeaders {
		endpoint: header(headers, "x-rivet-endpoint")
			.context("x-rivet-endpoint header is required")?,
		token: header(headers, "x-rivet-token"),
		pool_name,
		namespace: header(headers, "x-rivet-namespace-name")
			.context("x-rivet-namespace-name header is required")?,
	})
}

/// Strip the configured base path from the request path.
fn route_path(base_path: &str, path: &str) -> String {
	if path == base_path {
		return String::new();
	}
	let prefix = format!("{base_path}/");
	if let Some(rest) = path.strip_prefix(&prefix) {
		return format!("/{rest}");
	}
	path.to_string()
}

/// Build the axum app.
pub fn router(rt: Arc<ServerlessRuntime>) -> Router {
	Router::new().fallback_service(any(forward).with_state(rt))
}

/// Bind and serve until `shutdown` fires.
pub async fn serve(
	rt: Arc<ServerlessRuntime>,
	port: u16,
	shutdown: CancellationToken,
) -> Result<()> {
	// Bind dual-stack ([::]) so the front door accepts both IPv4 (as IPv4-mapped) and IPv6
	// loopback. The engine's metadata client may connect via `localhost`/::1; an IPv4-only
	// 0.0.0.0 bind would silently miss those.
	let addr = std::net::SocketAddr::from((std::net::Ipv6Addr::UNSPECIFIED, port));
	let listener = TcpListener::bind(addr)
		.await
		.with_context(|| format!("bind serverless listener on [::]:{port}"))?;
	tracing::info!(port, "container-runner serverless front door listening");
	let app = router(rt);
	let shutdown_fut = async move { shutdown.cancelled().await };
	axum::serve(listener, app.into_make_service())
		.with_graceful_shutdown(shutdown_fut)
		.await
		.context("axum::serve error")?;
	Ok(())
}

async fn forward(State(rt): State<Arc<ServerlessRuntime>>, req: Request) -> Response {
	let method = req.method().clone();
	let path = req.uri().path().to_string();
	tracing::info!(%method, %path, "front door received request");
	let subpath = route_path(&rt.settings.base_path, &path);

	match (method.as_str(), subpath.as_str()) {
		("GET", "") | ("GET", "/") => (
			StatusCode::OK,
			"This is a container-runner (Rivet serverless runner).\n",
		)
			.into_response(),
		("GET", "/health") => {
			let healthy = {
				let guard = rt.envoy.lock().await;
				guard.as_ref().map(|h| h.is_ping_healthy()).unwrap_or(true)
			};
			let status = if healthy {
				StatusCode::OK
			} else {
				StatusCode::SERVICE_UNAVAILABLE
			};
			let body = json!({ "status": if healthy { "ok" } else { "engine_ping_stale" }, "runtime": "container-runner", "version": PKG_VERSION });
			(status, axum::Json(body)).into_response()
		}
		("GET", "/metadata") => axum::Json(rt.metadata_json()).into_response(),
		("GET", "/metrics") => (StatusCode::OK, "# metrics not implemented\n").into_response(),
		("GET", "/start") | ("POST", "/start") => start(rt, req).await,
		("OPTIONS", _) => StatusCode::NO_CONTENT.into_response(),
		_ => (StatusCode::NOT_FOUND, "Not Found (container-runner)\n").into_response(),
	}
}

async fn start(rt: Arc<ServerlessRuntime>, req: Request) -> Response {
	let (parts, body) = req.into_parts();

	let headers = match parse_start_headers(&parts.headers) {
		Ok(h) => h,
		Err(e) => return (StatusCode::BAD_REQUEST, format!("{e}\n")).into_response(),
	};

	let body_bytes = match axum::body::to_bytes(body, rt.settings.max_start_payload_bytes).await {
		Ok(b) => b.to_vec(),
		Err(e) => {
			return (
				StatusCode::BAD_REQUEST,
				format!("failed to read body: {e}\n"),
			)
				.into_response();
		}
	};

	let handle = match rt.ensure_envoy(&headers).await {
		Ok(h) => h,
		Err(e) => return (StatusCode::BAD_REQUEST, format!("{e}\n")).into_response(),
	};

	let actor_start = match handle.decode_serverless_actor_start(&body_bytes) {
		Ok(s) => s,
		Err(e) => {
			return (
				StatusCode::BAD_REQUEST,
				format!("failed to decode start: {e}\n"),
			)
				.into_response();
		}
	};

	// SSE stream: keep the request open for the actor's lifetime.
	let (tx, rx) = mpsc::unbounded_channel::<Result<Vec<u8>, std::io::Error>>();
	let _ = tx.send(Ok(SSE_PING_FRAME.to_vec()));

	let rt_task = rt.clone();
	tokio::spawn(async move {
		if let Err(e) = handle.start_serverless_actor(&body_bytes).await {
			tracing::error!(error = ?e, actorId = %actor_start.actor_id, "start_serverless_actor failed");
			let _ = tx.send(Err(std::io::Error::other(format!("{e}"))));
			// A failed start leaves this instance poisoned; don't let it serve another actor.
			rt_task.request_exit(&actor_start.actor_id, "start_serverless_actor failed");
			return;
		}
		loop {
			tokio::select! {
				_ = handle.wait_actor_registered_then_stopped(&actor_start.actor_id, actor_start.generation) => {
					let _ = tx.send(Ok(SSE_STOPPING_FRAME.to_vec()));
					break;
				}
				_ = tokio::time::sleep(SSE_PING_INTERVAL) => {
					if tx.send(Ok(SSE_PING_FRAME.to_vec())).is_err() {
						break;
					}
				}
			}
		}
		// `/start` is ending. Dropping `tx` here ends the SSE body, which lets axum's
		// graceful shutdown complete; then the process exits.
		drop(tx);
		rt_task.request_exit(&actor_start.actor_id, "actor stopped");
	});

	let stream = UnboundedReceiverStream::new(rx).map(|chunk| chunk.map(Bytes::from));
	let mut resp = Response::new(Body::from_stream(stream));
	let h = resp.headers_mut();
	h.insert("content-type", "text/event-stream".parse().unwrap());
	h.insert("cache-control", "no-cache".parse().unwrap());
	h.insert("connection", "keep-alive".parse().unwrap());
	resp
}
