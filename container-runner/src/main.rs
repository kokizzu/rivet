//! `rivet-container-runner` — a Rivet serverless runner that hosts a single actor by
//! spawning a child game-server process and proxying Rivet's tunneled traffic to it.
//!
//! Serverless model: the engine's `POST /api/rivet/start` starts this container. On
//! actor start we spawn the child (`-- <command...>`), pipe its logs to stdout prefixed
//! with the actor id + key, and proxy inbound HTTP/WebSocket (arriving over Rivet's
//! tunnel) to the child's local port. On actor stop we SIGTERM the child.

mod callbacks;
mod child;
mod input;
mod inspector;
mod proxy;
mod serverless;

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

use crate::callbacks::{ContainerRunnerCallbacks, RunnerConfig};
use crate::serverless::{ServerlessRuntime, ServerlessSettings};

/// Default `/start` payload cap, matching the engine's default `maxStartPayloadBytes`.
const MAX_START_PAYLOAD_BYTES: usize = 1024 * 1024;

#[derive(Parser, Debug)]
#[command(
    name = "rivet-container-runner",
    about = "Rivet serverless runner: spawns a child game server and proxies tunneled traffic to it.",
    long_about = None,
)]
struct Args {
	/// Serverless HTTP front-door port. Rivet Compute injects RIVET_PORT (plain Cloud Run
	/// uses PORT); resolved in `main` as --port > RIVET_PORT > PORT > 8080.
	#[arg(long)]
	port: Option<u16>,

	/// Local port the child game server listens on (proxy target + child's $PORT).
	#[arg(long, env = "CHILD_PORT", default_value_t = 7770)]
	child_port: u16,

	/// Runner version reported to the engine (used for draining on deploy).
	#[arg(long, env = "RIVET_RUNNER_VERSION", default_value_t = 1)]
	runner_version: u32,

	/// Actor name this runner advertises/serves (repeatable).
	#[arg(long = "actor-name", env = "RIVET_ACTOR_NAME", default_value = "game")]
	actor_name: String,

	/// Base path the engine calls for serverless start.
	#[arg(long, env = "RIVET_SERVERLESS_BASE_PATH", default_value = "/api/rivet")]
	base_path: String,

	/// SIGTERM→SIGKILL grace period (seconds) when stopping the child.
	#[arg(long, env = "RIVET_STOP_GRACE_SECS", default_value_t = 25)]
	stop_grace_secs: u64,

	/// How long (seconds) to wait for the child's port to open before failing start.
	#[arg(long, env = "RIVET_READINESS_TIMEOUT_SECS", default_value_t = 30)]
	readiness_timeout_secs: u64,

	/// The child command to run, after `--`. e.g. `-- node /app/server.mjs`.
	#[arg(last = true, required = true)]
	command: Vec<String>,
}

fn main() -> Result<()> {
	// Rivet Compute documents RIVET_PORT for RivetKit apps. This runner is not a
	// standard RivetKit app, but accepting it makes the container friendlier there.
	if std::env::var_os("PORT").is_none() {
		if let Some(port) = std::env::var_os("RIVET_PORT") {
			// SAFETY: no other threads exist yet; the tokio runtime starts below.
			unsafe { std::env::set_var("PORT", port) };
		}
	}

	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()?
		.block_on(async_main())
}

async fn async_main() -> Result<()> {
	// Runner's own logs go to stderr; child logs go to stdout with the actor prefix.
	tracing_subscriber::fmt()
		.with_writer(std::io::stderr)
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
		)
		.init();

	let args = Args::parse();
	// Identifies this process (hence this container instance) across the log stream.
	// The logs carry no instance id, so this is how we tell "a new instance served the
	// next actor" apart from "the old instance was reused".
	let boot_id = crate::inspector::boot_id();
	tracing::info!(?args, %boot_id, "starting container-runner");

	// Front-door port: Rivet Compute injects RIVET_PORT; plain Cloud Run uses PORT.
	let port = args
		.port
		.or_else(|| env_u16("RIVET_PORT"))
		.or_else(|| env_u16("PORT"))
		.unwrap_or(8080);

	let runner_cfg = Arc::new(RunnerConfig {
		command_template: args.command.clone(),
		default_child_port: args.child_port,
		stop_grace: Duration::from_secs(args.stop_grace_secs),
		readiness_timeout: Duration::from_secs(args.readiness_timeout_secs),
	});

	let callbacks = Arc::new(ContainerRunnerCallbacks::new(runner_cfg));
	let shutdown_callbacks = callbacks.clone();

	let settings = ServerlessSettings {
		version: args.runner_version,
		base_path: args.base_path.clone(),
		actor_names: vec![args.actor_name.clone()],
		max_start_payload_bytes: MAX_START_PAYLOAD_BYTES,
	};

	let shutdown = CancellationToken::new();
	spawn_signal_handler(shutdown.clone());

	let rt = ServerlessRuntime::new(settings, callbacks, shutdown.clone());

	let serve_rt = rt.clone();
	let serve_shutdown = shutdown.clone();
	let server =
		tokio::spawn(async move { serverless::serve(serve_rt, port, serve_shutdown).await });

	// Wait for shutdown, then stop children and drain the envoy.
	shutdown.cancelled().await;
	tracing::info!("shutdown requested, stopping children + draining");
	shutdown_callbacks
		.stop_all(Duration::from_secs(args.stop_grace_secs))
		.await;
	rt.shutdown().await;

	match server.await {
		Ok(Ok(())) => {}
		Ok(Err(e)) => tracing::error!(error = ?e, "server error"),
		Err(e) => tracing::error!(error = ?e, "server task join error"),
	}

	tracing::info!("container-runner stopped");
	Ok(())
}

fn env_u16(key: &str) -> Option<u16> {
	std::env::var(key).ok().and_then(|s| s.parse().ok())
}

fn spawn_signal_handler(shutdown: CancellationToken) {
	tokio::spawn(async move {
		use tokio::signal::unix::{SignalKind, signal};
		let mut sigterm = signal(SignalKind::terminate()).expect("install SIGTERM handler");
		let mut sigint = signal(SignalKind::interrupt()).expect("install SIGINT handler");
		tokio::select! {
			_ = sigterm.recv() => tracing::info!("received SIGTERM"),
			_ = sigint.recv() => tracing::info!("received SIGINT"),
		}
		shutdown.cancel();
	});
}
