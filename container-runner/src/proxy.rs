//! Bridges Rivet's decoded tunnel traffic to the child game server's local port.
//!
//! The Rivet SDK reassembles tunnel frames into decoded `HttpRequest`s and
//! `WebSocketMessage` streams (see `EnvoyCallbacks::fetch` / `::websocket`). This
//! module forwards those to `127.0.0.1:<child_port>` and back — the ~200 lines of
//! glue the SDK deliberately leaves to the host.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex as TokioMutex;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;

use rivet_envoy_client::config::{
	BoxFuture, HttpRequest, HttpResponse, WebSocketHandler, WebSocketMessage, WebSocketSender,
};

/// Proxy a decoded inbound HTTP request to the child and map the response back.
pub async fn http_proxy(child_port: u16, req: HttpRequest) -> Result<HttpResponse> {
	let HttpRequest {
		method,
		path,
		headers,
		body,
		body_stream: _,
	} = req;

	let url = format!("http://127.0.0.1:{child_port}{path}");
	let method = reqwest::Method::from_bytes(method.as_bytes())?;

	let client = reqwest::Client::new();
	let mut rb = client.request(method, &url);
	for (k, v) in &headers {
		// Drop hop-by-hop / addressing headers that don't apply to the local hop.
		if k.eq_ignore_ascii_case("host") || k.eq_ignore_ascii_case("content-length") {
			continue;
		}
		rb = rb.header(k, v);
	}
	if let Some(body) = body {
		rb = rb.body(body);
	}

	let resp = rb.send().await?;
	let status = resp.status().as_u16();
	let mut out_headers = HashMap::new();
	for (k, v) in resp.headers() {
		let name = k.as_str();
		// Drop hop-by-hop / framing headers: the tunnel re-frames the body and sets its
		// own content-length, so forwarding the child's transfer-encoding/content-length
		// would conflict and truncate the response.
		if name.eq_ignore_ascii_case("transfer-encoding")
			|| name.eq_ignore_ascii_case("content-length")
			|| name.eq_ignore_ascii_case("connection")
		{
			continue;
		}
		if let Ok(s) = v.to_str() {
			out_headers.insert(name.to_string(), s.to_string());
		}
	}
	let bytes = resp.bytes().await?.to_vec();

	Ok(HttpResponse {
		status,
		headers: out_headers,
		body: Some(bytes),
		body_stream: None,
	})
}

/// Dial the child's WebSocket endpoint and return a handler that pumps frames in
/// both directions: child -> Rivet client (via `on_open`) and client -> child
/// (via `on_message` / `on_close`).
pub async fn ws_proxy(child_port: u16, path: String) -> Result<WebSocketHandler> {
	let url = format!("ws://127.0.0.1:{child_port}{path}");
	let (ws, _resp) = tokio_tungstenite::connect_async(&url).await?;
	let (sink, mut stream) = ws.split();
	let sink = Arc::new(TokioMutex::new(sink));

	// child -> client pump, started once the tunnel side opens.
	let on_open: Box<dyn FnOnce(WebSocketSender) -> BoxFuture<()> + Send> =
		Box::new(move |client: WebSocketSender| {
			Box::pin(async move {
				tokio::spawn(async move {
					while let Some(msg) = stream.next().await {
						match msg {
							Ok(Message::Binary(b)) => client.send(b.to_vec(), true),
							Ok(Message::Text(t)) => client.send_text(t.as_str()),
							Ok(Message::Close(cf)) => {
								let (code, reason) = match cf {
									Some(c) => {
										(Some(u16::from(c.code)), Some(c.reason.to_string()))
									}
									None => (None, None),
								};
								client.close(code, reason);
								break;
							}
							Ok(_) => {} // Ping/Pong/Frame handled by tungstenite
							Err(e) => {
								client.close(Some(1011), Some(format!("child ws error: {e}")));
								break;
							}
						}
					}
				});
			})
		});

	// client -> child
	let on_message: Box<dyn Fn(WebSocketMessage) -> BoxFuture<()> + Send + Sync> = {
		let sink = sink.clone();
		Box::new(move |msg: WebSocketMessage| {
			let sink = sink.clone();
			Box::pin(async move {
				let frame = if msg.binary {
					Message::Binary(msg.data.into())
				} else {
					Message::Text(String::from_utf8_lossy(&msg.data).into_owned().into())
				};
				let mut s = sink.lock().await;
				let _ = s.send(frame).await;
			})
		})
	};

	let on_close: Box<dyn Fn(u16, String) -> BoxFuture<()> + Send + Sync> = {
		let sink = sink.clone();
		Box::new(move |code: u16, reason: String| {
			let sink = sink.clone();
			Box::pin(async move {
				let frame = Message::Close(Some(CloseFrame {
					code: CloseCode::from(code),
					reason: reason.into(),
				}));
				let mut s = sink.lock().await;
				let _ = s.send(frame).await;
				let _ = s.close().await;
			})
		})
	};

	Ok(WebSocketHandler {
		on_message,
		on_close,
		on_open: Some(on_open),
	})
}
