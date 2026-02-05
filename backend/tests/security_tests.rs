use axum::http::StatusCode;
use axum_test::TestServer;
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, Mutex, RwLock};
use tunly::{
    create_app, AppState, AuthMode, ClientToServer, Metrics, ProxyResponse, ServerToClient,
    SessionState,
};

#[tokio::test]
async fn test_concurrent_proxy_flooding() {
    let state = Arc::new(AppState {
        _token: String::new(),
        req_id: AtomicU64::new(1),
        auth_mode: AuthMode::Ephemeral,
        jwt_secret: vec![0u8; 32],
        issued_tokens: Mutex::new(HashMap::new()),
        sessions: RwLock::new(HashMap::new()),
        rl: Mutex::new(HashMap::new()),
        proxy_rl: Mutex::new(HashMap::new()),
        allow_token_query: false,
        internal_key: None,
        metrics: Metrics::new(),
    });

    let (tx, mut rx) = mpsc::channel(100);
    let session = Arc::new(SessionState {
        outbound_tx: tx,
        pending: Mutex::new(HashMap::new()),
        _created_at: Instant::now(),
        last_seen: Mutex::new(Instant::now()),
        access_log: Mutex::new(Vec::new()),
    });

    let sid = "concurrent-test-session".to_string();
    state
        .sessions
        .write()
        .await
        .insert(sid.clone(), session.clone());

    // Background task to respond to all incoming proxy requests
    let session_clone = session.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                ServerToClient::ProxyRequest(req) => {
                    let mut pending = session_clone.pending.lock().await;
                    if let Some(otx) = pending.remove(&req.id) {
                        let _ = otx.send(ClientToServer::ProxyResponse(ProxyResponse {
                            id: req.id,
                            status: 200,
                            headers: vec![],
                            body_b64: String::new(),
                            is_compressed: false,
                        }));
                    }
                }
            }
        }
    });

    let app =
        create_app(state.clone()).into_make_service_with_connect_info::<std::net::SocketAddr>();
    let server = Arc::new(TestServer::new(app).unwrap());

    let mut handles = vec![];
    for _ in 0..50 {
        let server = server.clone();
        let sid = sid.clone();
        handles.push(tokio::spawn(async move {
            let url = format!("/s/{}/", sid);
            let response = server.get(&url).await;
            assert_eq!(response.status_code(), StatusCode::OK);
        }));
    }

    for h in handles {
        h.await.unwrap();
    }
}

#[tokio::test]
async fn test_server_health_check() {
    let state = Arc::new(AppState {
        _token: String::new(),
        req_id: AtomicU64::new(1),
        auth_mode: AuthMode::Ephemeral,
        jwt_secret: vec![0u8; 32],
        issued_tokens: Mutex::new(HashMap::new()),
        sessions: RwLock::new(HashMap::new()),
        rl: Mutex::new(HashMap::new()),
        proxy_rl: Mutex::new(HashMap::new()),
        allow_token_query: false,
        internal_key: None,
        metrics: Metrics::new(),
    });

    let app =
        create_app(state.clone()).into_make_service_with_connect_info::<std::net::SocketAddr>();
    let server = TestServer::new(app).unwrap();

    let response = server.get("/").await;
    // The root might return 404 or something else depending on routing
    // Let's just check if the server is up.
    assert!(response.status_code() != StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_proxy_rate_limiting() {
    let state = Arc::new(AppState {
        _token: String::new(),
        req_id: AtomicU64::new(1),
        auth_mode: AuthMode::Ephemeral,
        jwt_secret: vec![0u8; 32],
        issued_tokens: Mutex::new(HashMap::new()),
        sessions: RwLock::new(HashMap::new()),
        rl: Mutex::new(HashMap::new()),
        proxy_rl: Mutex::new(HashMap::new()),
        internal_key: None,
        allow_token_query: false,
        metrics: Metrics::new(),
    });

    let app =
        create_app(state.clone()).into_make_service_with_connect_info::<std::net::SocketAddr>();
    let server = TestServer::new(app).unwrap();

    // Send 120 requests to a proxy route.
    // They will fail with 503 "no tunnel client for session" but should still count towards rate limit.
    for _ in 0..120 {
        let response = server.get("/s/session123/").await;
        assert_eq!(response.status_code(), StatusCode::SERVICE_UNAVAILABLE);
    }

    // 121st request should be 429
    let response = server.get("/s/session123/").await;
    assert_eq!(response.status_code(), StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn test_body_size_limit() {
    let state = Arc::new(AppState {
        _token: String::new(),
        req_id: AtomicU64::new(1),
        auth_mode: AuthMode::Ephemeral,
        jwt_secret: vec![0u8; 32],
        issued_tokens: Mutex::new(HashMap::new()),
        sessions: RwLock::new(HashMap::new()),
        rl: Mutex::new(HashMap::new()),
        proxy_rl: Mutex::new(HashMap::new()),
        allow_token_query: false,
        internal_key: None,
        metrics: Metrics::new(),
    });

    // Mock a session to reach the body size limit check in proxy_logic
    let (tx, _rx) = mpsc::channel(1);
    let session = Arc::new(SessionState {
        outbound_tx: tx,
        pending: Mutex::new(HashMap::new()),
        _created_at: Instant::now(),
        last_seen: Mutex::new(Instant::now()),
        access_log: Mutex::new(Vec::new()),
    });

    state
        .sessions
        .write()
        .await
        .insert("test-session".to_string(), session);

    let app =
        create_app(state.clone()).into_make_service_with_connect_info::<std::net::SocketAddr>();
    let server = TestServer::new(app).unwrap();

    // Create a 2.1 MB body
    let large_body = vec![0u8; 2100 * 1024];

    let response = server
        .post("/s/test-session/")
        .add_header(axum::http::header::CONTENT_TYPE, "application/octet-stream")
        .json(&large_body)
        .await;

    assert_eq!(response.status_code(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_jwt_auth_flow() {
    let state = Arc::new(AppState {
        _token: String::new(),
        req_id: AtomicU64::new(1),
        auth_mode: AuthMode::Ephemeral,
        jwt_secret: vec![0u8; 32],
        issued_tokens: Mutex::new(HashMap::new()),
        sessions: RwLock::new(HashMap::new()),
        rl: Mutex::new(HashMap::new()),
        proxy_rl: Mutex::new(HashMap::new()),
        allow_token_query: true,
        internal_key: None,
        metrics: Metrics::new(),
    });

    let app =
        create_app(state.clone()).into_make_service_with_connect_info::<std::net::SocketAddr>();
    let server = TestServer::new(app).unwrap();

    // 1. Get Token
    let token_resp = server.get("/token").await;
    assert_eq!(token_resp.status_code(), StatusCode::OK);
    let body: tunly::TokenResponse = token_resp.json();
    let token = body.token;
    let sid = body.session;

    // 2. Validate token in WS upgrade (using query param since we enabled it)
    // Note: TestServer doesn't easily support actual WS upgrades in this context,
    // but it triggers the route handler.
    // Handlers that return response (like ws_handler) will return UNAUTHORIZED if token fails.

    let ws_url = format!("/ws?sid={}&token={}", sid, token);
    let ws_resp = server
        .get(&ws_url)
        .add_header(axum::http::header::UPGRADE, "websocket")
        .add_header(axum::http::header::CONNECTION, "upgrade")
        .add_header(
            axum::http::header::SEC_WEBSOCKET_KEY,
            "dGhlIHNhbXBsZSBub25jZQ==",
        )
        .add_header(axum::http::header::SEC_WEBSOCKET_VERSION, "13")
        .await;
    // If it gets past auth, it will try to upgrade.
    // In TestServer, if it returns 200 or 101, it means it passed auth.
    // However, ws_handler returns `Response` which might be 101 Switching Protocols.
    assert!(
        ws_resp.status_code() == StatusCode::SWITCHING_PROTOCOLS
            || ws_resp.status_code() == StatusCode::OK
    );

    // 3. Try to reuse same token (should fail)
    let ws_resp_retry = server
        .get(&ws_url)
        .add_header(axum::http::header::UPGRADE, "websocket")
        .add_header(axum::http::header::CONNECTION, "upgrade")
        .add_header(
            axum::http::header::SEC_WEBSOCKET_KEY,
            "dGhlIHNhbXBsZSBub25jZQ==",
        )
        .add_header(axum::http::header::SEC_WEBSOCKET_VERSION, "13")
        .await;
    assert_eq!(ws_resp_retry.status_code(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_compression_utils() {
    let large_data = "Tunly compression test ".repeat(100);
    let (b64, compressed) = tunly::compress_body(large_data.as_bytes());
    assert!(compressed, "Data should be compressed");

    let decompressed = tunly::decompress_body(&b64, compressed);
    assert_eq!(large_data.as_bytes(), decompressed.as_slice());
}

#[test]
fn test_no_compression_for_small_data() {
    let small_data = b"small data";
    let (b64, compressed) = tunly::compress_body(small_data);
    assert!(!compressed, "Small data should not be compressed");

    let decompressed = tunly::decompress_body(&b64, compressed);
    assert_eq!(small_data, decompressed.as_slice());
}
