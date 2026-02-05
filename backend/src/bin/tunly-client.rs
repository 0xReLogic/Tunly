use std::{
    fs,
    io::{self, Write},
    time::{Duration, Instant},
};

use base64::{engine::general_purpose, Engine as _};
use clap::Parser;
use futures::{SinkExt, StreamExt};
use rand::RngCore;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::{Error as WsError, Message};
use tunly::{ClientToServer, ProxyRequest, ProxyResponse, ServerToClient};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct TokenSession {
    token: String,
    // Accept both `session` and `sid` keys from the token endpoint
    #[serde(default, alias = "sid")]
    session: String,
    #[serde(default)]
    expires_in: u64,
}

#[derive(Parser, Debug, Clone)]
#[command(name = "tunly-client", about = "Tunly Client")]
struct ClientArgs {
    /// Remote server host[:port], default app.tunly.online (backend)
    #[arg(long)]
    remote_host: Option<String>,

    /// Local target host:port to forward to, e.g. 127.0.0.1:80
    #[arg(long, default_value = "127.0.0.1:80")]
    local: String,

    /// Use secure WebSocket (wss). Accepts explicit boolean: --use-wss=false
    #[arg(long, action = clap::ArgAction::Set, default_value_t = true)]
    use_wss: bool,

    /// WebSocket path on server
    #[arg(long, default_value = "/ws")]
    path: String,

    /// Optional: URL to fetch token (JSON {token, session, expires_in} or plain text). Useful for ephemeral tokens, e.g. https://app.tunly.online/token
    #[arg(long)]
    token_url: Option<String>,
}

fn generate_session_id() -> String {
    let mut bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut bytes);
    general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = ClientArgs::parse();
    // Initialize Rustls crypto provider (required for rustls 0.23 when using ring)
    let _ = rustls::crypto::ring::default_provider().install_default();

    tracing::info!("Running Tunly Client. Press Ctrl+C to exit.");

    let http = reqwest::Client::builder()
        .build()
        .expect("failed to build http client");

    // Resolve remote host and scheme
    let remote_host = args
        .remote_host
        .clone()
        .unwrap_or_else(|| "app.tunly.online".to_string());
    let scheme = if args.use_wss { "wss" } else { "ws" };
    let path = if args.path.starts_with('/') {
        args.path.clone()
    } else {
        format!("/{}", args.path)
    };

    // Acquire token/session
    let mut token_session = if let Some(url) = args.token_url.clone() {
        match http.get(&url).send().await {
            Ok(resp) => {
                match resp.error_for_status() {
                    Ok(ok) => {
                        let ctype = ok
                            .headers()
                            .get(reqwest::header::CONTENT_TYPE)
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or("")
                            .to_string();
                        let bytes = ok.bytes().await.unwrap_or_default();
                        let body_str = String::from_utf8_lossy(&bytes);
                        if ctype.contains("application/json")
                            || body_str.trim_start().starts_with('{')
                        {
                            match serde_json::from_slice::<TokenSession>(&bytes) {
                                Ok(mut ts) => {
                                    if ts.token.trim().is_empty() {
                                        tracing::error!("token-url JSON missing token, prompting manual token...");
                                        ts.token = String::new();
                                    }
                                    ts
                                }
                                Err(e) => {
                                    tracing::error!("failed to parse token-url JSON: {}. prompting manual token...", e);
                                    TokenSession::default()
                                }
                            }
                        } else {
                            let txt = body_str.trim().to_string();
                            if txt.is_empty() {
                                tracing::error!(
                                    "token-url returned empty body, prompting manual token..."
                                );
                                TokenSession::default()
                            } else {
                                TokenSession {
                                    token: txt,
                                    ..Default::default()
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("token-url error: {}. prompting manual token...", e);
                        TokenSession::default()
                    }
                }
            }
            Err(e) => {
                tracing::error!(
                    "failed to fetch token-url: {}. prompting manual token...",
                    e
                );
                TokenSession::default()
            }
        }
    } else {
        // Try env/config; if not found, leave token empty to trigger prompt later
        match load_token() {
            Ok(tok) => TokenSession {
                token: tok,
                ..Default::default()
            },
            Err(_) => TokenSession::default(),
        }
    };

    let mut attempt: u32 = 0;

    loop {
        // If session is still missing (e.g. manual token), generate one now.
        if token_session.session.trim().is_empty() {
            token_session.session = generate_session_id();
        }

        // If token missing, try auto-fetch from token-url first (Ephemeral mode)
        if token_session.token.trim().is_empty() {
            if let Some(url) = args.token_url.clone() {
                match http.get(&url).send().await {
                    Ok(resp) => match resp.error_for_status() {
                        Ok(ok) => {
                            let ctype = ok
                                .headers()
                                .get(reqwest::header::CONTENT_TYPE)
                                .and_then(|v| v.to_str().ok())
                                .unwrap_or("")
                                .to_string();
                            let bytes = ok.bytes().await.unwrap_or_default();
                            let body_str = String::from_utf8_lossy(&bytes);
                            if ctype.contains("application/json")
                                || body_str.trim_start().starts_with('{')
                            {
                                if let Ok(ts) = serde_json::from_slice::<TokenSession>(&bytes) {
                                    if !ts.token.trim().is_empty() {
                                        token_session = ts;
                                        // proceed to connect with freshly fetched token
                                        // (skip manual prompt)
                                    } else {
                                        tracing::error!("token-url JSON missing token, falling back to manual prompt...");
                                    }
                                } else {
                                    tracing::error!("failed to parse token-url JSON, falling back to manual prompt...");
                                }
                            } else {
                                let txt = body_str.trim().to_string();
                                if !txt.is_empty() {
                                    token_session.token = txt;
                                } else {
                                    tracing::error!("token-url returned empty body, falling back to manual prompt...");
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("token-url error: {}. falling back to manual prompt...", e);
                        }
                    },
                    Err(e) => {
                        tracing::error!(
                            "failed to fetch token-url: {}. falling back to manual prompt...",
                            e
                        );
                    }
                }
            }

            if token_session.token.trim().is_empty() {
                println!(
                    "Enter token (if you don't have one, visit https://{})",
                    remote_host
                );
                print!("token: ");
                let _ = io::stdout().flush();
                let mut buf = String::new();
                if io::stdin().read_line(&mut buf).is_err() {
                    continue;
                }
                token_session.token = buf.trim().to_string();
                if token_session.token.is_empty() {
                    continue;
                }
            }
        }

        // Build current ws URL with session
        let ws_url = format!(
            "{}://{}{}?sid={}",
            scheme, remote_host, path, token_session.session
        );

        attempt += 1;
        tracing::info!("Connecting to {} (attempt #{})...", ws_url, attempt);
        if attempt == 1 {
            tracing::info!("If server is not running yet, please wait. Waking up server...");
        }

        let mut req = ws_url
            .clone()
            .into_client_request()
            .expect("failed to build WS request");
        req.headers_mut().insert(
            "Authorization",
            format!("Bearer {}", token_session.token).parse().unwrap(),
        );
        // Enable WebSocket compression (permessage-deflate)
        req.headers_mut().insert(
            "Sec-WebSocket-Extensions",
            "permessage-deflate; client_max_window_bits".parse().unwrap(),
        );

        match tokio_tungstenite::connect_async(req).await {
            Ok((ws_stream, resp)) => {
                let compressed = resp
                    .headers()
                    .get("Sec-WebSocket-Extensions")
                    .map(|v| v.to_str().unwrap_or("").contains("permessage-deflate"))
                    .unwrap_or(false);

                if compressed {
                    tracing::info!("Connected! WebSocket compression: ENABLED");
                } else {
                    tracing::info!("Connected! WebSocket compression: DISABLED (not supported by server)");
                }

                // Token valid; ask for local address before starting proxying
                let default_local = args.local.clone();
                let input_prompt = format!("Enter local address (default {}): ", default_local);
                print!("{}", input_prompt);
                let _ = io::stdout().flush();
                let mut line = String::new();
                let _ = io::stdin().read_line(&mut line);
                let line = line.trim();
                let local = if line.is_empty() {
                    default_local
                } else {
                    line.to_string()
                };
                let local_base = format!("http://{}", local);

                let public_http = if scheme == "wss" {
                    format!("https://{}/s/{}/", remote_host, token_session.session)
                } else {
                    format!("http://{}/s/{}/", remote_host, token_session.session)
                };
                tracing::info!("Public URL: {}", public_http);
                if token_session.expires_in > 0 {
                    tracing::info!("Note: token expires in ~{}s", token_session.expires_in);
                }

                tracing::info!("Connected. Waiting for requests...");
                tracing::info!("Press Ctrl+C to exit.");
                let (mut ws_tx, mut ws_rx) = ws_stream.split();

                // Outbound single-writer task with channel
                let (out_tx, mut out_rx) = mpsc::channel::<Message>(64);
                let writer = tokio::spawn(async move {
                    while let Some(msg) = out_rx.recv().await {
                        if ws_tx.send(msg).await.is_err() {
                            break;
                        }
                    }
                });

                // Heartbeat: ping every 20s via the outbound channel
                let hb_tx = out_tx.clone();
                let heartbeat = tokio::spawn(async move {
                    let mut interval = tokio::time::interval(Duration::from_secs(20));
                    loop {
                        interval.tick().await;
                        if hb_tx.send(Message::Ping(Vec::new().into())).await.is_err() {
                            break;
                        }
                    }
                });

                while let Some(msg_res) = ws_rx.next().await {
                    let msg = match msg_res {
                        Ok(m) => m,
                        Err(e) => {
                            tracing::error!("WebSocket error: {}", e);
                            break;
                        }
                    };
                    match msg {
                        Message::Text(txt) => match serde_json::from_str::<ServerToClient>(&txt) {
                            Ok(ServerToClient::ProxyRequest(req_msg)) => {
                                let resp_msg = handle_proxy(&http, &local_base, req_msg).await;
                                let text =
                                    serde_json::to_string(&ClientToServer::ProxyResponse(resp_msg))
                                        .expect("serialize response");
                                if let Err(e) = out_tx.send(Message::Text(text.into())).await {
                                    tracing::error!("Failed to send response over WS: {}", e);
                                    break;
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to parse server message: {}", e);
                            }
                        },
                        Message::Ping(p) => {
                            let _ = out_tx.send(Message::Pong(p)).await;
                        }
                        Message::Close(_) => {
                            println!("Server closed connection");
                            break;
                        }
                        _ => {}
                    }
                }

                heartbeat.abort();
                writer.abort();

                // After a disconnect, generate a new session ID for next attempt
                token_session.session = generate_session_id();
                // Reset attempts so backoff starts small again
                attempt = 0;
            }
            Err(e) => {
                if let WsError::Http(resp) = &e {
                    let code = resp.status().as_u16();
                    if code == 401 || code == 403 {
                        println!("Token is invalid or has expired.");
                        println!(
                            "Get a new token at https://{} and enter it again.",
                            remote_host
                        );
                        token_session.token.clear();
                        // Reset attempt for fresh start after reprompt
                        attempt = 0;
                        continue;
                    }
                }
                tracing::error!("Failed to connect: {}", e);
                if attempt <= 2 {
                    tracing::info!("Server might be cold starting. Please wait...");
                }
                // Exponential backoff before reconnect (max 15s)
                let backoff = 2u64.saturating_pow(attempt.min(4));
                sleep(Duration::from_secs(backoff.min(15))).await;
                // Refresh session for next attempt
                token_session.session = generate_session_id();
            }
        }
    }
}

async fn handle_proxy(
    http: &reqwest::Client,
    local_base: &str,
    req_msg: ProxyRequest,
) -> ProxyResponse {
    tracing::info!("-> CLIENT received proxy request for URI: {}", &req_msg.uri);
    // Build URL to local server
    let url = if req_msg.uri.starts_with('/') {
        format!("{}{}", local_base, req_msg.uri)
    } else {
        format!("{}/{}", local_base.trim_end_matches('/'), req_msg.uri)
    };

    let method = req_msg.method.as_str();
    let start = Instant::now();

    let mut builder = http.request(
        reqwest::Method::from_bytes(method.as_bytes()).unwrap_or(reqwest::Method::GET),
        &url,
    );

    // Headers
    let mut headers = HeaderMap::new();
    for (k, v) in req_msg.headers.iter() {
        if is_hop_by_hop(k) {
            continue;
        }
        if k.eq_ignore_ascii_case("host") {
            // Rewrite host to local target
            let local_host = local_base
                .trim_start_matches("http://")
                .trim_start_matches("https://");
            let host_parts: Vec<&str> = local_host.split(':').collect();
            let port = host_parts.get(1).unwrap_or(&"80");
            if let Ok(val) = HeaderValue::from_str(&format!("localhost:{}", port)) {
                headers.insert(HeaderName::from_static("host"), val);
            }
            continue;
        }
        if let (Ok(name), Ok(value)) = (HeaderName::try_from(k.as_str()), HeaderValue::from_str(v))
        {
            headers.insert(name, value);
        }
    }
    builder = builder.headers(headers);

    // Body
    let body = tunly::decompress_body(&req_msg.body_b64, req_msg.is_compressed);
    if !body.is_empty() {
        builder = builder.body(body);
    }

    // Do request
    let result = builder.send().await;

    match result {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let resp_headers = headers_to_vec(resp.headers());
            let bytes = resp.bytes().await.unwrap_or_default();
            let (body_b64, is_compressed) = tunly::compress_body(&bytes);
            let dur_ms = start.elapsed().as_millis();
            tracing::info!(
                "LOCAL {} {} -> {} in {}ms",
                method, req_msg.uri, status, dur_ms
            );
            ProxyResponse {
                id: req_msg.id,
                status,
                headers: resp_headers,
                body_b64,
                is_compressed,
            }
        }
        Err(err) => {
            let msg = format!("upstream error: {}", err);
            let dur_ms = start.elapsed().as_millis();
            tracing::info!(
                "LOCAL {} {} -> 502 in {}ms ({})",
                method, req_msg.uri, dur_ms, err
            );
            let (body_b64, is_compressed) = tunly::compress_body(msg.as_bytes());
            ProxyResponse {
                id: req_msg.id,
                status: 502,
                headers: vec![("content-type".into(), "text/plain".into())],
                body_b64,
                is_compressed,
            }
        }
    }
}

fn headers_to_vec(headers: &HeaderMap) -> Vec<(String, String)> {
    headers
        .iter()
        .filter_map(|(k, v)| {
            if is_hop_by_hop(k.as_str()) {
                return None;
            }
            Some((k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
        })
        .collect()
}

fn is_hop_by_hop(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailers"
            | "transfer-encoding"
            | "upgrade"
    )
}

fn load_token() -> Result<String, String> {
    // Try env first (TUNLY_TOKEN only)
    if let Ok(tok) = std::env::var("TUNLY_TOKEN") {
        return Ok(tok);
    }

    let content = fs::read_to_string("config.txt").map_err(|e| e.to_string())?;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let lower = line.to_lowercase();
        if lower.starts_with("token") {
            // tolerate typos like "tokenn"
            // Try separators
            if let Some(pos) = line.find(':') {
                return Ok(line[pos + 1..].trim().trim_matches('"').to_string());
            } else if let Some(pos) = line.find('=') {
                return Ok(line[pos + 1..].trim().trim_matches('"').to_string());
            } else {
                // token <value>
                let mut parts = line.split_whitespace();
                let _ = parts.next();
                if let Some(val) = parts.next() {
                    return Ok(val.trim().trim_matches('"').to_string());
                }
            }
        }
    }
    Err("token not found in env TUNLY_TOKEN or config.txt".into())
}
