use std::{fs, time::Duration};

use base64::{engine::general_purpose, Engine as _};
use clap::Parser;
use futures::{SinkExt, StreamExt};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;

#[derive(Parser, Debug, Clone)]
#[command(name = "tunly-client", about = "Tunly Client")] 
struct ClientArgs {
    /// Remote server host:port, e.g. 1.2.3.4:9000
    #[arg(long)]
    remote_host: String,

    /// Local target host:port to forward to, e.g. 127.0.0.1:80
    #[arg(long, default_value = "127.0.0.1:80")]
    local: String,

    /// Use secure WebSocket (wss)
    #[arg(long, default_value_t = false)]
    use_wss: bool,

    /// WebSocket path on server
    #[arg(long, default_value = "/ws")]
    path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")] 
pub enum ServerToClient {
    ProxyRequest(ProxyRequest),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")] 
pub enum ClientToServer {
    ProxyResponse(ProxyResponse),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyRequest {
    pub id: u64,
    pub method: String,
    pub uri: String,
    pub headers: Vec<(String, String)>,
    pub body_b64: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyResponse {
    pub id: u64,
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body_b64: String,
}

#[tokio::main]
async fn main() {
    let args = ClientArgs::parse();

    let token = load_token().unwrap_or_else(|e| {
        eprintln!("Failed to load token from config.txt: {}", e);
        std::process::exit(1);
    });

    let scheme = if args.use_wss { "wss" } else { "ws" };
    let token_enc = urlencoding::encode(&token);
    let path = if args.path.starts_with('/') { args.path.clone() } else { format!("/{}", args.path) };
    let ws_url = format!("{}://{}{}?token={}", scheme, args.remote_host, path, token_enc);

    let local_base = format!("http://{}", args.local);
    let http = reqwest::Client::builder()
        .no_gzip()
        .build()
        .expect("failed to build http client");

    let mut attempt: u32 = 0;
    loop {
        attempt += 1;
        println!("Connecting to {} (attempt #{})...", ws_url, attempt);

        match tokio_tungstenite::connect_async(&ws_url).await {
            Ok((ws_stream, _resp)) => {
                println!("Connected. Waiting for requests...");
                let (mut ws_tx, mut ws_rx) = ws_stream.split();

                while let Some(msg_res) = ws_rx.next().await {
                    let msg = match msg_res {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("WebSocket error: {}", e);
                            break;
                        }
                    };
                    match msg {
                        Message::Text(txt) => {
                            match serde_json::from_str::<ServerToClient>(&txt) {
                                Ok(ServerToClient::ProxyRequest(req_msg)) => {
                                    let resp_msg = handle_proxy(&http, &local_base, req_msg).await;
                                    let text = serde_json::to_string(&ClientToServer::ProxyResponse(resp_msg))
                                        .expect("serialize response");
                                    if let Err(e) = ws_tx.send(Message::Text(text)).await {
                                        eprintln!("Failed to send response over WS: {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to parse server message: {}", e);
                                }
                            }
                        }
                        Message::Ping(p) => {
                            let _ = ws_tx.send(Message::Pong(p)).await;
                        }
                        Message::Close(_) => {
                            println!("Server closed connection");
                            break;
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to connect: {}", e);
            }
        }

        // Backoff before reconnect
        sleep(Duration::from_secs(2)).await;
    }
}

async fn handle_proxy(http: &reqwest::Client, local_base: &str, req_msg: ProxyRequest) -> ProxyResponse {
    // Build URL to local server
    let url = if req_msg.uri.starts_with('/') {
        format!("{}{}", local_base, req_msg.uri)
    } else {
        format!("{}/{}", local_base.trim_end_matches('/'), req_msg.uri)
    };

    let method = req_msg.method.as_str();

    let mut builder = http.request(
        reqwest::Method::from_bytes(method.as_bytes()).unwrap_or(reqwest::Method::GET),
        &url,
    );

    // Headers
    let mut headers = HeaderMap::new();
    for (k, v) in req_msg.headers.iter() {
        if is_hop_by_hop(k) { continue; }
        if k.eq_ignore_ascii_case("host") {
            // Rewrite host to local target
            if let Ok(val) = HeaderValue::from_str(local_base.trim_start_matches("http://").trim_start_matches("https://")) {
                headers.insert(HeaderName::from_static("host"), val);
            }
            continue;
        }
        if let (Ok(name), Ok(value)) = (
            HeaderName::try_from(k.as_str()),
            HeaderValue::from_str(v),
        ) { headers.insert(name, value); }
    }
    builder = builder.headers(headers);

    // Body
    let body = general_purpose::STANDARD_NO_PAD
        .decode(req_msg.body_b64.as_bytes())
        .unwrap_or_default();
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
            let body_b64 = general_purpose::STANDARD_NO_PAD.encode(&bytes);
            ProxyResponse { id: req_msg.id, status, headers: resp_headers, body_b64 }
        }
        Err(err) => {
            let msg = format!("upstream error: {}", err);
            ProxyResponse { id: req_msg.id, status: 502, headers: vec![("content-type".into(), "text/plain".into())], body_b64: general_purpose::STANDARD_NO_PAD.encode(msg.as_bytes()) }
        }
    }
}

fn headers_to_vec(headers: &HeaderMap) -> Vec<(String, String)> {
    headers
        .iter()
        .filter_map(|(k, v)| {
            if is_hop_by_hop(k.as_str()) { return None; }
            Some((k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
        })
        .collect()
}

fn is_hop_by_hop(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "connection" | "keep-alive" | "proxy-authenticate" | "proxy-authorization" | "te" | "trailers" | "transfer-encoding" | "upgrade"
    )
}

fn load_token() -> Result<String, String> {
    // Try env first (TUNLY_TOKEN only)
    if let Ok(tok) = std::env::var("TUNLY_TOKEN") { return Ok(tok); }

    let content = fs::read_to_string("config.txt").map_err(|e| e.to_string())?;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let lower = line.to_lowercase();
        if lower.starts_with("token") { // tolerate typos like "tokenn"
            // Try separators
            if let Some(pos) = line.find(':') {
                return Ok(line[pos+1..].trim().trim_matches('"').to_string());
            } else if let Some(pos) = line.find('=') {
                return Ok(line[pos+1..].trim().trim_matches('"').to_string());
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
