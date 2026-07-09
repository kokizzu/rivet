//! Custom Rivet actor inspector page, served directly by the container-runner.
//!
//! Two responsibilities:
//!  1. On actor start, ensure an inspector bearer token exists in KV key `[0x03]`
//!     (base64 `"Aw=="`). The dashboard reads this key BEFORE mounting the inspector
//!     iframe; if it is absent the inspector panel stays blank.
//!  2. Serve a minimal "Hello World" HTML page for `GET /inspector/ui/...`, intercepted
//!     in `callbacks::fetch` before the request would otherwise be proxied to the child.
//!
//! This intentionally does NOT implement the inspector WebSocket / BARE protocol.

use std::collections::HashMap;
use std::io::Read;

use rivet_envoy_client::config::HttpResponse;
use rivet_envoy_client::handle::EnvoyHandle;

const INSPECTOR_TOKEN_KEY: &[u8] = &[0x03];
const INSPECTOR_UI_PREFIX: &str = "/inspector/ui/";

/// True if a request path should be served by our inspector UI instead of proxied to
/// the child.
pub fn is_inspector_ui_path(path: &str) -> bool {
	// Ignore any query string when matching.
	let path = path.split(['?', '#']).next().unwrap_or(path);
	path.starts_with(INSPECTOR_UI_PREFIX)
}

/// Custom inspector page. It MUST postMessage `{type:"ready"}` to the parent on
/// load, otherwise the dashboard keeps the iframe hidden and errors out after ~8s.
/// It optionally listens for the host's `{type:"init", v:1, auth:<token>}` message and
/// shows the token — purely informational, safe to ignore.
const INSPECTOR_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<title>Actor Inspector</title>
<style>
  body { font-family: system-ui, sans-serif; margin: 2rem; }
  #token { font-family: monospace; word-break: break-all; color: #555; }
</style>
</head>
<body>
  <h1>Hello World</h1>
  <p id="token">waiting for host init&hellip;</p>
  <script>
    function post(msg) { try { window.parent.postMessage(msg, "*"); } catch (e) {} }

    function boot() {
      // 1) Unhide the iframe. The host validates with
      //    z.object({ type: z.literal("ready"), v: z.literal(1) }) — omitting `v` makes
      //    the safeParse fail SILENTLY, so the iframe stays hidden and errors after ~8s.
      post({ type: "ready", v: 1 });
      // 2) Declare that we support tabs, but advertise none. The host shows no chips and
      //    just renders this page. To add one later, push an entry shaped
      //    { id, label, icon } — all three required. `icon` must be one of the known ids
      //    (workflow, database, state, queue, plug, terminal, tag, logs, comments,
      //    folder-tree, microchip, wrench, hard-drive, box-archive) or it renders as `?`.
      //    Leave `isCustom` off: a custom tab makes the host navigate the iframe to
      //    /inspector/custom-tabs/{id}/, which we would then also have to serve.
      post({ type: "tabs-available", v: 1, tabs: [] });
    }
    if (document.readyState === "complete") boot();
    else window.addEventListener("load", boot);

    // The host follows up with { type:"init", v:1, actorId, authToken, theme, activeTab }.
    window.addEventListener("message", function (ev) {
      var d = ev.data;
      if (!d) return;
      if (d.type === "init") {
        document.getElementById("token").textContent =
          d.authToken ? ("auth token: " + d.authToken) : "init received (no auth)";
      }
      // Single tab: nothing to switch. `set-active-tab` would land here.
      // if (d.type === "set-active-tab") { render(d.tab); }
    });
  </script>
</body>
</html>
"#;

/// Build the inspector HTML response. Always `200 text/html`.
pub fn inspector_response() -> HttpResponse {
	let mut headers = HashMap::new();
	headers.insert(
		"content-type".to_string(),
		"text/html; charset=utf-8".to_string(),
	);
	HttpResponse {
		status: 200,
		headers,
		body: Some(INSPECTOR_HTML.as_bytes().to_vec()),
		body_stream: None,
	}
}

/// `GET /metadata` on the actor surface. The dashboard polls this; without a handler it
/// falls through to the child proxy.
pub fn is_metadata_path(path: &str) -> bool {
	let path = path.split('?').next().unwrap_or(path);
	path == "/metadata" || path == "/metadata/"
}

/// Mirrors rivetkit's actor-surface handler (`registry/http.rs: handle_metadata_fetch`).
pub fn metadata_response() -> HttpResponse {
	let mut headers = HashMap::new();
	headers.insert("content-type".to_string(), "application/json".to_string());
	let body = br#"{"runtime":"rivetkit","version":"2.3.0"}"#.to_vec();
	HttpResponse {
		status: 200,
		headers,
		body: Some(body),
		body_stream: None,
	}
}

/// Stable per-process identifier, generated once. The runner is PID 1 in the image, so
/// one boot id == one container instance. Logged at startup and on every actor start so
/// an actor can be attributed to an instance (the log stream carries no instance id).
pub fn boot_id() -> &'static str {
	static BOOT_ID: std::sync::OnceLock<String> = std::sync::OnceLock::new();
	BOOT_ID.get_or_init(|| {
		// 9 random bytes -> 12 chars. Falls back to a fixed marker if the CSPRNG is
		// unavailable, which would itself be worth seeing in the logs.
		let mut buf = [0u8; 9];
		match std::fs::File::open("/dev/urandom").and_then(|mut f| f.read_exact(&mut buf)) {
			Ok(()) => base64url_nopad(&buf),
			Err(_) => "no-urandom".to_string(),
		}
	})
}

/// Generate a URL-safe base64 (no padding) token from ~32 random bytes.
fn generate_token() -> anyhow::Result<String> {
	let mut buf = [0u8; 32];
	// No `rand`/`getrandom` in this crate's dep tree; read the OS CSPRNG directly.
	// This runner only ever runs on Linux (Cloud Run), where /dev/urandom is present.
	let mut f = std::fs::File::open("/dev/urandom")?;
	f.read_exact(&mut buf)?;
	Ok(base64url_nopad(&buf))
}

/// Minimal URL-safe base64 encoder (RFC 4648 §5) without padding. Kept local to avoid
/// adding a dependency just for the inspector token.
fn base64url_nopad(input: &[u8]) -> String {
	const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
	let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
	for chunk in input.chunks(3) {
		let b0 = chunk[0] as u32;
		let b1 = *chunk.get(1).unwrap_or(&0) as u32;
		let b2 = *chunk.get(2).unwrap_or(&0) as u32;
		let n = (b0 << 16) | (b1 << 8) | b2;
		out.push(ALPHABET[((n >> 18) & 0x3f) as usize] as char);
		out.push(ALPHABET[((n >> 12) & 0x3f) as usize] as char);
		if chunk.len() > 1 {
			out.push(ALPHABET[((n >> 6) & 0x3f) as usize] as char);
		}
		if chunk.len() > 2 {
			out.push(ALPHABET[(n & 0x3f) as usize] as char);
		}
	}
	out
}

pub async fn ensure_inspector_token(handle: &EnvoyHandle, actor_id: &str) -> anyhow::Result<()> {
	let existing = handle
		.kv_get(actor_id.to_string(), vec![INSPECTOR_TOKEN_KEY.to_vec()])
		.await?;

	if matches!(existing.first(), Some(Some(_))) {
		// Token already present — leave it untouched so we don't rotate on restart.
		return Ok(());
	}

	let token = generate_token()?;
	handle
		.kv_put(
			actor_id.to_string(),
			vec![(INSPECTOR_TOKEN_KEY.to_vec(), token.into_bytes())],
		)
		.await?;
	tracing::info!(actorId = %actor_id, "wrote inspector token to KV key [0x03]");
	Ok(())
}

#[cfg(test)]
#[path = "../tests/inline/inspector.rs"]
mod tests;
