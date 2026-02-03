use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use axum::{
    extract::{
        connect_info::ConnectInfo,
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    http::{HeaderMap, Request, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{any, get},
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use clap::Parser;
use futures::{SinkExt, StreamExt};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot, Mutex, RwLock};
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::trace::TraceLayer;

// Simple per-IP rate limit for /token: 10 requests per 60 seconds
const RL_WINDOW_SECS: u64 = 60;
const RL_MAX_PER_WINDOW: u32 = 10;
// Session idle TTL (seconds) before being GC-removed if no activity
const SESSION_IDLE_TTL_SECS: u64 = 600;

#[derive(Parser, Debug, Clone)]
#[command(name = "tunly-server", about = "Tunly Server")]
struct ServerArgs {
    /// Host to bind, e.g. 0.0.0.0
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to bind, e.g. 8080
    #[arg(long, env = "PORT", default_value_t = 8080)]
    port: u16,

    /// Legacy combined bind (overrides host/port if provided), e.g. 0.0.0.0:9000
    #[arg(long)]
    bind: Option<String>,

    /// Authentication token required by client (or use env TUNLY_TOKEN)
    #[arg(long)]
    token: Option<String>,

    /// Allow token via query parameter for WS (not recommended). Default: false
    #[arg(long, default_value_t = false)]
    allow_token_query: bool,
}

#[derive(Debug, Clone)]
enum AuthMode {
    Fixed(String),
    Ephemeral,
}

#[derive(Debug, Clone)]
struct AccessLogEntry {
    method: String,
    uri: String,
    status: u16,
    dur_ms: u128,
}

#[derive(Debug)]
struct SessionState {
    outbound_tx: mpsc::Sender<ServerToClient>,
    pending: Mutex<HashMap<u64, oneshot::Sender<ClientToServer>>>,
    _created_at: Instant,
    last_seen: Mutex<Instant>,
    access_log: Mutex<Vec<AccessLogEntry>>, // ring buffer (last N)
}

#[derive(Debug)]
struct AppState {
    // For Fixed mode this holds the configured token; empty string in Ephemeral mode
    _token: String,
    req_id: AtomicU64,
    // Auth
    auth_mode: AuthMode,
    // token -> (ip, expiry, session)
    issued_tokens: Mutex<HashMap<String, (String, Instant, String)>>,
    // session -> state
    sessions: RwLock<HashMap<String, Arc<SessionState>>>,
    // rate limit map: ip -> (count, window_start)
    rl: Mutex<HashMap<String, (u32, Instant)>>,
    // config: allow token in query string for WS
    allow_token_query: bool,
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

    // Auth mode: if --token or TUNLY_TOKEN provided => Fixed, else Ephemeral tokens via /token
    let auth_mode = if let Some(t) = args
        .token
        .clone()
        .or_else(|| std::env::var("TUNLY_TOKEN").ok())
    {
        AuthMode::Fixed(t)
    } else {
        AuthMode::Ephemeral
    };

    let state = Arc::new(AppState {
        _token: match &auth_mode {
            AuthMode::Fixed(t) => t.clone(),
            AuthMode::Ephemeral => String::new(),
        },
        req_id: AtomicU64::new(1),
        auth_mode: auth_mode.clone(),
        issued_tokens: Mutex::new(HashMap::new()),
        sessions: RwLock::new(HashMap::new()),
        rl: Mutex::new(HashMap::new()),
        allow_token_query: args.allow_token_query,
    });

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/token", get(token_endpoint))
        .route("/healthz", get(health))
        .route("/_next/*path", any(next_asset_redirect))
        .route("/s/:sid/_log", get(session_log))
        .route("/s/:sid/", any(proxy_handler_root))
        .route("/s/:sid", any(proxy_handler_root))
        .route("/s/:sid/*path", any(proxy_handler_path))
        .fallback(fallback_404)
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    // Background GC: periodically prune expired ephemeral tokens
    {
        let gc_state = state.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(60));
            loop {
                ticker.tick().await;
                let now = Instant::now();
                let mut issued = gc_state.issued_tokens.lock().await;
                let before = issued.len();
                issued.retain(|_, (_ip, exp, _sid)| *exp > now);
                let removed = before.saturating_sub(issued.len());
                if removed > 0 {
                    eprintln!("GC: removed {} expired token(s)", removed);
                }
            }
        });
    }

    // Background GC: remove idle sessions
    {
        let gc_state = state.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(60));
            loop {
                ticker.tick().await;
                let now = Instant::now();
                // snapshot
                let entries: Vec<(String, Arc<SessionState>)> = {
                    let sessions = gc_state.sessions.read().await;
                    sessions
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect()
                };
                let mut to_remove = Vec::new();
                for (sid, sess) in entries {
                    let last = *sess.last_seen.lock().await;
                    if now.duration_since(last).as_secs() >= SESSION_IDLE_TTL_SECS {
                        to_remove.push(sid);
                    }
                }
                if !to_remove.is_empty() {
                    let mut sessions = gc_state.sessions.write().await;
                    let mut removed = 0usize;
                    for sid in to_remove {
                        if sessions.remove(&sid).is_some() {
                            removed += 1;
                        }
                    }
                    if removed > 0 {
                        eprintln!("GC: removed {} stale session(s)", removed);
                    }
                }
            }
        });
    }

    let bind_str = if let Some(b) = args.bind.clone() {
        b
    } else {
        format!("{}:{}", args.host, args.port)
    };
    let addr: SocketAddr = bind_str
        .parse()
        .expect("--bind must be like 0.0.0.0:9000 or use --host/--port");

    println!("Tunly Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let svc = app.into_make_service_with_connect_info::<SocketAddr>();
    axum::serve(listener, svc).await.unwrap();
}

fn extract_real_ip(addr: &SocketAddr, headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string())
}

async fn ws_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let sid = match params.get("sid") {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return (StatusCode::BAD_REQUEST, "missing sid").into_response(),
    };

    // Extract token, prefer Authorization header; only allow query token if explicitly enabled
    let auth_header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let bearer = auth_header.strip_prefix("Bearer ");
    let token_str = if let Some(tok) = bearer {
        Some(tok.to_string())
    } else if state.allow_token_query {
        params.get("token").cloned()
    } else {
        None
    };
    let Some(token) = token_str else {
        let msg = if state.allow_token_query {
            "missing token"
        } else {
            "missing token (use Authorization: Bearer <token>)"
        };
        return (StatusCode::UNAUTHORIZED, msg).into_response();
    };

    let token_ok = match &state.auth_mode {
        AuthMode::Fixed(expected) => token == *expected,
        AuthMode::Ephemeral => {
            println!(
                "WS_HANDLER Check: IP={}, Headers={:?}",
                extract_real_ip(&addr, &headers),
                headers
            );
            let ip = extract_real_ip(&addr, &headers);
            let mut issued = state.issued_tokens.lock().await;
            if let Some((tok_ip, exp, session)) = issued.get(&token) {
                if *tok_ip == ip && *exp > Instant::now() && *session == sid {
                    issued.remove(&token);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
    };

    if !token_ok {
        return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
    }

    ws.on_upgrade(move |socket| client_ws(socket, state, sid))
}

async fn token_endpoint(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Response {
    // Only available in Ephemeral mode
    match &state.auth_mode {
        AuthMode::Fixed(_) => {
            return (StatusCode::FORBIDDEN, "token issuance disabled").into_response();
        }
        AuthMode::Ephemeral => {}
    }

    // Rate limiting per IP
    let ip = extract_real_ip(&addr, &headers);
    println!("TOKEN_ENDPOINT: IP={}, Headers={:?}", ip, headers);
    let now = Instant::now();
    {
        let mut rl = state.rl.lock().await;
        use std::collections::hash_map::Entry;
        match rl.entry(ip.clone()) {
            Entry::Occupied(mut e) => {
                let (ref mut count, ref mut start) = *e.get_mut();
                let elapsed = now.duration_since(*start).as_secs();
                if elapsed >= RL_WINDOW_SECS {
                    *count = 1;
                    *start = now;
                } else if *count >= RL_MAX_PER_WINDOW {
                    let retry_after = RL_WINDOW_SECS - elapsed;
                    return axum::http::Response::builder()
                        .status(StatusCode::TOO_MANY_REQUESTS)
                        .header(
                            axum::http::header::CONTENT_TYPE,
                            "text/plain; charset=utf-8",
                        )
                        .header("retry-after", retry_after.to_string())
                        .header("cache-control", "no-store")
                        .header("x-robots-tag", "noindex, nofollow")
                        .header("referrer-policy", "no-referrer")
                        .body(axum::body::Body::from("rate limit exceeded for /token"))
                        .unwrap();
                } else {
                    *count += 1;
                }
            }
            Entry::Vacant(v) => {
                v.insert((1, now));
            }
        }
    }

    // Generate random token & session, tie to requesting IP, TTL 5 minutes
    let mut tok_bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut tok_bytes);
    let token = general_purpose::URL_SAFE_NO_PAD.encode(tok_bytes);

    let mut sid_bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut sid_bytes);
    let session = general_purpose::URL_SAFE_NO_PAD.encode(sid_bytes);

    let expiry = Instant::now() + Duration::from_secs(300);

    {
        let mut issued = state.issued_tokens.lock().await;
        issued.insert(token.clone(), (ip, expiry, session.clone()));
    }

    // Return JSON with security headers
    let body = format!(
        r#"{{"token":"{}","session":"{}","expires_in":{}}}"#,
        token, session, 300
    );
    axum::http::Response::builder()
        .status(StatusCode::OK)
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .header("cache-control", "no-store")
        .header("x-robots-tag", "noindex, nofollow")
        .header("referrer-policy", "same-origin")
        .body(axum::body::Body::from(body))
        .unwrap()
}

async fn client_ws(stream: WebSocket, state: Arc<AppState>, sid: String) {
    println!("Client connected via WebSocket for session {}", sid);

    let (mut ws_tx, mut ws_rx) = stream.split();

    // Channel for outbound messages (server -> client)
    let (out_tx, mut out_rx) = mpsc::channel::<ServerToClient>(64);

    // Create session state and store
    let session_state = Arc::new(SessionState {
        outbound_tx: out_tx.clone(),
        pending: Mutex::new(HashMap::new()),
        _created_at: Instant::now(),
        last_seen: Mutex::new(Instant::now()),
        access_log: Mutex::new(Vec::new()),
    });
    {
        let mut sessions = state.sessions.write().await;
        sessions.insert(sid.clone(), session_state.clone());
    }

    // Task: forward outbound messages to websocket
    let write_session = session_state.clone();
    let write_task = tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            let text = serde_json::to_string(&msg).unwrap();
            if ws_tx.send(Message::Text(text)).await.is_err() {
                break;
            }
            // update last_seen on outbound activity
            {
                let mut ls = write_session.last_seen.lock().await;
                *ls = Instant::now();
            }
        }
    });

    // Task: read inbound messages from websocket (responses from client)
    let read_state = state.clone();
    let read_sid = sid.clone();
    let read_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_rx.next().await {
            // update last_seen on any inbound WS message
            if let Some(sess) = { read_state.sessions.read().await.get(&read_sid).cloned() } {
                let mut ls = sess.last_seen.lock().await;
                *ls = Instant::now();
            }
            if let Message::Text(txt) = msg {
                match serde_json::from_str::<ClientToServer>(&txt) {
                    Ok(ClientToServer::ProxyResponse(resp)) => {
                        let maybe_session =
                            { read_state.sessions.read().await.get(&read_sid).cloned() };
                        if let Some(sess) = maybe_session {
                            let mut pending = sess.pending.lock().await;
                            if let Some(tx) = pending.remove(&resp.id) {
                                let _ = tx.send(ClientToServer::ProxyResponse(resp));
                            }
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

    // Remove session on disconnect
    {
        let mut sessions = state.sessions.write().await;
        sessions.remove(&sid);
    }

    println!("Client disconnected for session {}", sid);
}

async fn health() -> &'static str {
    "ok"
}

async fn session_log(Path(sid): Path<String>, State(state): State<Arc<AppState>>) -> Response {
    let maybe = { state.sessions.read().await.get(&sid).cloned() };
    let Some(sess) = maybe else {
        return (StatusCode::NOT_FOUND, "session not found").into_response();
    };

    let log = sess.access_log.lock().await.clone();
    let mut html = String::from("<!doctype html><meta charset=\"utf-8\"><title>Tunly Session Log</title><style>body{font-family:system-ui,-apple-system,Segoe UI,Roboto,Ubuntu,\"Helvetica Neue\",Arial,sans-serif;padding:20px}table{border-collapse:collapse;width:100%}th,td{border:1px solid #ddd;padding:8px}th{background:#f7f7f7;text-align:left}code{background:#f3f3f3;padding:2px 4px;border-radius:3px}</style>");
    html.push_str(&format!("<h1>Session <code>{}</code></h1>", sid));
    html.push_str(&format!("<p>Quick links: <a href=\"/s/{}/\">/</a> · <a href=\"/s/{}/api\">/api</a> · <a href=\"/s/{}/blog\">/blog</a></p>", sid, sid, sid));
    html.push_str("<table><thead><tr><th>Method</th><th>URI</th><th>Status</th><th>Duration</th></tr></thead><tbody>");
    for e in log.iter().rev() {
        // newest first
        html.push_str(&format!(
            "<tr><td>{}</td><td><code>{}</code></td><td>{}</td><td>{} ms</td></tr>",
            escape_html(&e.method),
            escape_html(&e.uri),
            e.status,
            e.dur_ms
        ));
    }
    html.push_str("</tbody></table>");

    axum::http::Response::builder()
        .status(StatusCode::OK)
        .header(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")
        .header("cache-control", "no-store")
        .header("x-robots-tag", "noindex, nofollow")
        .header("referrer-policy", "same-origin")
        .body(axum::body::Body::from(html))
        .unwrap()
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

async fn proxy_handler_root(
    Path(sid): Path<String>,
    State(state): State<Arc<AppState>>,
    req: Request<axum::body::Body>,
) -> Response {
    proxy_logic(State(state), sid, "".to_string(), req).await
}

async fn proxy_handler_path(
    Path((sid, path)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    req: Request<axum::body::Body>,
) -> Response {
    proxy_logic(State(state), sid, path, req).await
}

async fn proxy_logic(
    State(state): State<Arc<AppState>>,
    sid: String,
    path: String,
    req: Request<axum::body::Body>,
) -> Response {
    println!("-> PROXY_HANDLER: sid='{}', path='{}'", sid, path);
    let start = Instant::now();

    // Prepare request snapshot pieces up front
    let method = req.method().to_string();
    let uri: Uri = req.uri().clone();
    // Build URI for client: "/" + tail + optional ?query
    let tail = path.trim_start_matches('/');
    let mut uri_str = if tail.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", tail)
    };
    if let Some(query) = uri.query() {
        uri_str.push('?');
        uri_str.push_str(query);
    }

    // Lookup session
    let maybe_sess = { state.sessions.read().await.get(&sid).cloned() };
    let Some(sess) = maybe_sess else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "no tunnel client for session",
        )
            .into_response();
    };

    // mark activity
    {
        let mut ls = sess.last_seen.lock().await;
        *ls = Instant::now();
    }

    // Build request snapshot
    let id = state.req_id.fetch_add(1, Ordering::SeqCst);

    let headers_vec = headers_to_vec(req.headers());

    let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .unwrap_or_default();
    let body_b64 = general_purpose::STANDARD_NO_PAD.encode(&body_bytes);

    let proxy_req = ProxyRequest {
        id,
        method: method.clone(),
        uri: uri_str.clone(),
        headers: headers_vec,
        body_b64,
    };

    // Prepare oneshot for the response
    let (resp_tx, resp_rx) = oneshot::channel::<ClientToServer>();
    {
        let mut pending = sess.pending.lock().await;
        pending.insert(id, resp_tx);
    }

    // Send to client
    if sess
        .outbound_tx
        .send(ServerToClient::ProxyRequest(proxy_req))
        .await
        .is_err()
    {
        let mut pending = sess.pending.lock().await;
        pending.remove(&id);
        // log failure
        let dur_ms = start.elapsed().as_millis();
        {
            let mut log = sess.access_log.lock().await;
            log.push(AccessLogEntry {
                method: method.clone(),
                uri: uri_str.clone(),
                status: StatusCode::BAD_GATEWAY.as_u16(),
                dur_ms,
            });
            if log.len() > 50 {
                let drop_n = log.len() - 50;
                log.drain(0..drop_n);
            }
        }
        println!(
            "PROXY {} {} -> {} in {}ms (sid={})",
            method,
            uri_str,
            StatusCode::BAD_GATEWAY.as_u16(),
            dur_ms,
            sid
        );
        return (StatusCode::BAD_GATEWAY, "failed to send to tunnel client").into_response();
    }

    // Await response with timeout
    let resp = match tokio::time::timeout(std::time::Duration::from_secs(30), resp_rx).await {
        Ok(Ok(ClientToServer::ProxyResponse(r))) => r,
        Ok(Err(_)) => {
            let dur_ms = start.elapsed().as_millis();
            {
                let mut log = sess.access_log.lock().await;
                log.push(AccessLogEntry {
                    method: method.clone(),
                    uri: uri_str.clone(),
                    status: StatusCode::BAD_GATEWAY.as_u16(),
                    dur_ms,
                });
                if log.len() > 50 {
                    let drop_n = log.len() - 50;
                    log.drain(0..drop_n);
                }
            }
            println!(
                "PROXY {} {} -> {} in {}ms (sid={})",
                method,
                uri_str,
                StatusCode::BAD_GATEWAY.as_u16(),
                dur_ms,
                sid
            );
            return (StatusCode::BAD_GATEWAY, "tunnel closed").into_response();
        }
        Err(_) => {
            // Timeout
            let mut pending = sess.pending.lock().await;
            pending.remove(&id);
            let dur_ms = start.elapsed().as_millis();
            {
                let mut log = sess.access_log.lock().await;
                log.push(AccessLogEntry {
                    method: method.clone(),
                    uri: uri_str.clone(),
                    status: StatusCode::GATEWAY_TIMEOUT.as_u16(),
                    dur_ms,
                });
                if log.len() > 50 {
                    let drop_n = log.len() - 50;
                    log.drain(0..drop_n);
                }
            }
            println!(
                "PROXY {} {} -> {} in {}ms (sid={})",
                method,
                uri_str,
                StatusCode::GATEWAY_TIMEOUT.as_u16(),
                dur_ms,
                sid
            );
            return (StatusCode::GATEWAY_TIMEOUT, "upstream timeout").into_response();
        }
    };

    // Build response to external client
    let mut builder = axum::http::Response::builder().status(resp.status);
    for (k, v) in resp.headers.iter() {
        // Skip hop-by-hop headers
        if is_hop_by_hop(k) {
            continue;
        }

        // Rewrite relative Location headers to stay under /s/:sid/
        if k.eq_ignore_ascii_case("location") {
            // Absolute-path Location: rewrite under /s/:sid/
            if v.starts_with('/') {
                let new_loc = if v.starts_with(&format!("/s/{}/", sid)) {
                    v.clone()
                } else {
                    format!("/s/{}/{}", sid, v.trim_start_matches('/'))
                };
                if let (Ok(name), Ok(value)) = (
                    axum::http::header::HeaderName::from_bytes(k.as_bytes()),
                    axum::http::HeaderValue::from_str(&new_loc),
                ) {
                    builder = builder.header(name, value);
                }
                continue;
            }
            // Absolute-URL Location (http/https): strip scheme+host and rewrite path+query under /s/:sid/
            let lower = v.to_ascii_lowercase();
            if lower.starts_with("http://") || lower.starts_with("https://") {
                if let Some(scheme_idx) = v.find("://") {
                    let after_scheme = scheme_idx + 3;
                    if let Some(path_rel_idx) = v[after_scheme..].find('/') {
                        let path_start = after_scheme + path_rel_idx; // index of '/'
                        let path_q = &v[path_start..]; // includes leading '/'
                        let new_loc = if path_q.starts_with(&format!("/s/{}/", sid)) {
                            path_q.to_string()
                        } else {
                            format!("/s/{}/{}", sid, path_q.trim_start_matches('/'))
                        };
                        if let (Ok(name), Ok(value)) = (
                            axum::http::header::HeaderName::from_bytes(k.as_bytes()),
                            axum::http::HeaderValue::from_str(&new_loc),
                        ) {
                            builder = builder.header(name, value);
                        }
                        continue;
                    }
                }
            }
        }

        if let (Ok(name), Ok(value)) = (
            axum::http::header::HeaderName::from_bytes(k.as_bytes()),
            axum::http::HeaderValue::from_str(v),
        ) {
            builder = builder.header(name, value);
        }
    }
    // Security/cache headers to reduce leakage
    builder = builder
        .header("cache-control", "no-store")
        .header("x-robots-tag", "noindex, nofollow")
        .header("referrer-policy", "same-origin");
    // Persist session id to a cookie for asset routing (/_next/* -> /s/:sid/_next/*)
    if let Ok(cv) = axum::http::HeaderValue::from_str(&format!(
        "tunly_sid={}; Path=/; Max-Age=600; HttpOnly; SameSite=Lax",
        sid
    )) {
        builder = builder.header(axum::http::header::SET_COOKIE, cv);
    }

    let body: Vec<u8> = general_purpose::STANDARD_NO_PAD
        .decode(resp.body_b64.as_bytes())
        .unwrap_or_default();
    let response = builder
        .body(axum::body::Body::from(body))
        .unwrap()
        .into_response();

    // lightweight logging
    let dur_ms = start.elapsed().as_millis();
    {
        // push to session ring buffer (keep last 50)
        let mut log = sess.access_log.lock().await;
        log.push(AccessLogEntry {
            method: method.clone(),
            uri: uri_str.clone(),
            status: response.status().as_u16(),
            dur_ms,
        });
        if log.len() > 50 {
            let drop_n = log.len() - 50;
            log.drain(0..drop_n);
        }
    }
    println!(
        "PROXY {} {} -> {} in {}ms (sid={})",
        method,
        uri_str,
        response.status().as_u16(),
        dur_ms,
        sid
    );

    response
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

// Redirect root Next.js asset requests (/_next/*) to the prefixed session path (/s/:sid/_next/*)
// We infer the session id preferring Referer (so multiple sessions work), then cookie.
async fn next_asset_redirect(Path(path): Path<String>, headers: HeaderMap, uri: Uri) -> Response {
    let qs = uri
        .query()
        .map(|q| {
            let mut s = String::from("?");
            s.push_str(q);
            s
        })
        .unwrap_or_default();
    // 1) Prefer Referer: derive from path like https://host/s/<sid>/...
    if let Some(ref_val) = headers
        .get(axum::http::header::REFERER)
        .and_then(|v| v.to_str().ok())
    {
        if let Some(sid) = sid_from_referer(ref_val) {
            let new_loc = format!("/s/{}/_next/{}{}", sid, path, qs);
            let hv = axum::http::HeaderValue::from_str(&new_loc)
                .unwrap_or(axum::http::HeaderValue::from_static("/"));
            return axum::http::Response::builder()
                .status(StatusCode::TEMPORARY_REDIRECT)
                .header(axum::http::header::LOCATION, hv)
                .header("cache-control", "no-store")
                .body(axum::body::Body::empty())
                .unwrap()
                .into_response();
        }
    }
    // 2) Fallback to cookie
    if let Some(sid) = cookie_value(&headers, "tunly_sid") {
        let new_loc = format!("/s/{}/_next/{}{}", sid, path, qs);
        let hv = axum::http::HeaderValue::from_str(&new_loc)
            .unwrap_or(axum::http::HeaderValue::from_static("/"));
        return axum::http::Response::builder()
            .status(StatusCode::TEMPORARY_REDIRECT)
            .header(axum::http::header::LOCATION, hv)
            .header("cache-control", "no-store")
            .body(axum::body::Body::empty())
            .unwrap()
            .into_response();
    }
    (
        StatusCode::NOT_FOUND,
        "not found: /_next/* (no session context)",
    )
        .into_response()
}

fn sid_from_referer(referer: &str) -> Option<String> {
    // look for "/s/" then capture until next '/'
    if let Some(idx) = referer.find("/s/") {
        let after = &referer[idx + 3..];
        let end = after.find('/').unwrap_or(after.len());
        let sid = &after[..end];
        if !sid.is_empty() {
            return Some(sid.to_string());
        }
    }
    None
}

fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    let raw = headers.get(axum::http::header::COOKIE)?.to_str().ok()?;
    for part in raw.split(';') {
        let kv = part.trim();
        if let Some(val) = kv.strip_prefix(&format!("{}=", name)) {
            return Some(val.to_string());
        }
    }
    None
}

// Fallback for unmatched routes: return 404 and include the requested URI for visibility
async fn fallback_404(uri: Uri) -> Response {
    let body = format!("not found: {}", uri);
    axum::http::Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("cache-control", "no-store")
        .body(axum::body::Body::from(body))
        .unwrap()
        .into_response()
}
