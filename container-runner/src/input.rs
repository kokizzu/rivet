//! Decoding of the opaque `ActorConfig.input` bytes into a concrete child-process
//! launch spec.
//!
//! The Rivet runner protocol only carries an actor's `name`, `key`, `create_ts`
//! and an opaque `input: Option<Vec<u8>>`. Everything the game server needs to
//! launch (command, args, env, port) is therefore JSON-encoded by the client into
//! `input` and decoded here. All fields are optional; anything omitted falls back
//! to the CLI-provided template (`rivet-container-runner -- <command...>`).

use serde::Deserialize;
use std::collections::HashMap;

/// Shape of the JSON carried in `ActorConfig.input`.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActorInput {
	/// Overrides the CLI command template entirely (program + fixed args).
	#[serde(default)]
	pub command: Option<Vec<String>>,
	/// Extra args appended after the command template / `command`.
	#[serde(default)]
	pub args: Vec<String>,
	/// Extra environment variables for the child process.
	#[serde(default)]
	pub env: HashMap<String, String>,
	/// Local port the child listens on; also exported to the child as `PORT`.
	/// Falls back to the runner's `--child-port` when omitted.
	#[serde(default)]
	pub port: Option<u16>,
}

impl ActorInput {
	/// Parse the opaque input bytes. An absent/empty payload yields defaults.
	pub fn parse(input: Option<&[u8]>) -> anyhow::Result<Self> {
		match input {
			None => Ok(Self::default()),
			Some(bytes) if bytes.is_empty() => Ok(Self::default()),
			Some(bytes) => serde_json::from_slice(bytes)
				.map_err(|e| anyhow::anyhow!("failed to parse actor input as JSON: {e}")),
		}
	}
}
