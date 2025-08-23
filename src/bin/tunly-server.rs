use std::{collections::HashMap, net::SocketAddr, sync::{Arc, atomic::{AtomicU64, Ordering}}};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    http::{HeaderMap, Request, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{any, get},
    Router,
};
use clap::Parser;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, Mutex, RwLock, oneshot};
use base64::{engine::general_purpose, Engine as _};

#[derive(Parser, Debug, Clone)]
#[command(name = "tunly-server", about = "Tunly Server")] 
struct ServerArgs {
    /// Host to bind, e.g. 0.0.0.0
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to bind, e.g. 9000
    #[arg(long, env = "PORT", default_value_t = 9000)]
    port: u16,

    /// Legacy combined bind (overrides host/port if provided), e.g. 0.0.0.0:9000
    #[arg(long)]
    bind: Option<String>,

    /// Authentication token required by client (or use env TUNLY_TOKEN)
    #[arg(long)]
    token: Option<String>,
}

#[derive(Debug)]
struct AppState {
    token: String,
    outbound_tx: RwLock<Option<mpsc::Sender<ServerToClient>>>,
    pending: Mutex<HashMap<u64, oneshot::Sender<ClientToServer>>>,
    req_id: AtomicU64,
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
    let args = ServerArgs::parse();

    // Resolve token: CLI > TUNLY_TOKEN
    let effective_token = args
        .token
        .or_else(|| std::env::var("TUNLY_TOKEN").ok())
        .expect("Token is required. Provide --token or set TUNLY_TOKEN env");

    let state = Arc::new(AppState {
        token: effective_token.clone(),
        outbound_tx: RwLock::new(None),
        pending: Mutex::new(HashMap::new()),
        req_id: AtomicU64::new(1),
    });

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/healthz", get(health))
        .route("/*path", any(proxy_handler))
        .with_state(state.clone());

    let bind_str = if let Some(b) = args.bind.clone() { b } else { format!("{}:{}", args.host, args.port) };
    let addr: SocketAddr = bind_str
        .parse()
        .expect("--bind must be like 0.0.0.0:9000 or use --host/--port");

    println!("Tunly Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let token_ok = params
        .get("token")
        .map(|t| t == &state.token)
        .unwrap_or(false);

    if !token_ok {
        return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
    }

    ws.on_upgrade(move |socket| client_ws(socket, state))
}

async fn client_ws(stream: WebSocket, state: Arc<AppState>) {
    println!("Client connected via WebSocket");

    let (mut ws_tx, mut ws_rx) = stream.split();

    // Channel for outbound messages (server -> client)
    let (out_tx, mut out_rx) = mpsc::channel::<ServerToClient>(64);

    // Store sender so HTTP handlers can forward requests
    {
        let mut guard = state.outbound_tx.write().await;
        *guard = Some(out_tx.clone());
    }

    // Task: forward outbound messages to websocket
    let write_task = tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            let text = serde_json::to_string(&msg).unwrap();
            if ws_tx.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
    });

    // Task: read inbound messages from websocket (responses from client)
    let read_state = state.clone();
    let read_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_rx.next().await {
            if let Message::Text(txt) = msg {
                match serde_json::from_str::<ClientToServer>(&txt) {
                    Ok(ClientToServer::ProxyResponse(resp)) => {
                        let mut pending = read_state.pending.lock().await;
                        if let Some(tx) = pending.remove(&resp.id) {
                            let _ = tx.send(ClientToServer::ProxyResponse(resp));
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse client message: {}", e);
                    }
                }
            }
        }
    });

    // Wait for either side to finish (disconnect)
    let _ = tokio::join!(write_task, read_task);

    // Clear outbound sender on disconnect
    {
        let mut guard = state.outbound_tx.write().await;
        *guard = None;
    }

    println!("Client disconnected");
}

async fn health() -> &'static str {
    "ok"
}

async fn proxy_handler(
    State(state): State<Arc<AppState>>,
    Path(_path): Path<String>,
    mut req: Request<axum::body::Body>,
) -> Response {
    // Check if a client is connected
    let maybe_tx = { state.outbound_tx.read().await.clone() };
    let Some(tx) = maybe_tx else {
        return (StatusCode::SERVICE_UNAVAILABLE, "no tunnel client connected").into_response();
    };

    // Build request snapshot
    let id = state.req_id.fetch_add(1, Ordering::SeqCst);

    let method = req.method().to_string();
    let uri: Uri = req.uri().clone();
    let uri_str = uri.to_string();

    let headers_vec = headers_to_vec(req.headers());

    let body_owned = std::mem::take(req.body_mut());
    let body_bytes = match axum::body::to_bytes(body_owned, 2 * 1024 * 1024).await { // 2MB limit
        Ok(b) => b,
        Err(_) => return (StatusCode::PAYLOAD_TOO_LARGE, "body too large").into_response(),
    };
    let body_b64 = general_purpose::STANDARD_NO_PAD.encode(&body_bytes);

    let proxy_req = ProxyRequest {
        id,
        method,
        uri: uri_str,
        headers: headers_vec,
        body_b64,
    };

    // Prepare oneshot for the response
    let (resp_tx, resp_rx) = oneshot::channel::<ClientToServer>();
    {
        let mut pending = state.pending.lock().await;
        pending.insert(id, resp_tx);
    }

    // Send to client
    if tx.send(ServerToClient::ProxyRequest(proxy_req)).await.is_err() {
        let mut pending = state.pending.lock().await;
        pending.remove(&id);
        return (StatusCode::BAD_GATEWAY, "failed to send to tunnel client").into_response();
    }

    // Await response with timeout
    let resp = match tokio::time::timeout(std::time::Duration::from_secs(30), resp_rx).await {
        Ok(Ok(ClientToServer::ProxyResponse(r))) => r,
        Ok(Err(_)) => return (StatusCode::BAD_GATEWAY, "tunnel closed").into_response(),
        Err(_) => {
            // Timeout
            let mut pending = state.pending.lock().await;
            pending.remove(&id);
            return (StatusCode::GATEWAY_TIMEOUT, "upstream timeout").into_response();
        }
    };

    // Build response to external client
    let mut builder = axum::http::Response::builder().status(resp.status);
    for (k, v) in resp.headers.iter() {
        // Skip hop-by-hop headers
        if is_hop_by_hop(k) { continue; }
        if let (Ok(name), Ok(value)) = (
            axum::http::header::HeaderName::from_bytes(k.as_bytes()),
            axum::http::HeaderValue::from_str(v),
        ) {
            builder = builder.header(name, value);
        }
    }
    let body = match general_purpose::STANDARD_NO_PAD.decode(resp.body_b64.as_bytes()) {
        Ok(b) => b,
        Err(_) => Vec::new(),
    };
    builder.body(axum::body::Body::from(body)).unwrap().into_response()
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
