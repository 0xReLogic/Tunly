use std::{
    collections::HashMap,
    net::SocketAddr,
    path::PathBuf,
    sync::{atomic::AtomicU64, Arc},
    time::{Duration, Instant},
};

use clap::Parser;
use futures::StreamExt;
use rand::Rng;
use rustls_acme::{caches::DirCache, AcmeConfig};
use tokio::sync::{Mutex, RwLock};
use tunly::{AppState, AuthMode, Metrics, SESSION_IDLE_TTL_SECS};

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

    /// Custom JWT secret key. If not provided, a random one is generated on startup.
    #[arg(long, env = "TUNLY_JWT_SECRET")]
    jwt_secret: Option<String>,

    /// Allow token via query parameter for WS (not recommended). Default: false
    #[arg(long, default_value_t = false)]
    allow_token_query: bool,

    /// (Optional) Internal key to restrict /token access (env: TUNLY_INTERNAL_KEY)
    #[arg(long, env = "TUNLY_INTERNAL_KEY")]
    internal_key: Option<String>,

    /// Domain(s) for automatic Let's Encrypt certificates via TLS-ALPN.
    #[arg(long = "acme-domain", value_delimiter = ',')]
    acme_domains: Vec<String>,

    /// Email address(es) for ACME account contact information.
    #[arg(long = "acme-email", value_delimiter = ',')]
    acme_emails: Vec<String>,

    /// Use the Let's Encrypt staging environment instead of production.
    #[arg(long, default_value_t = false)]
    acme_staging: bool,

    /// Persistent ACME account and certificate cache directory.
    #[arg(long, default_value = "./.tunly/acme")]
    acme_cache: PathBuf,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = ServerArgs::parse();
    let _ = rustls::crypto::ring::default_provider().install_default();

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

    let jwt_secret = if let Some(secret) = args.jwt_secret {
        secret.into_bytes()
    } else {
        let mut secret = vec![0u8; 32];
        rand::rng().fill_bytes(&mut secret);
        secret
    };

    let state = Arc::new(AppState {
        _token: match &auth_mode {
            AuthMode::Fixed(t) => t.clone(),
            AuthMode::Ephemeral => String::new(),
        },
        req_id: AtomicU64::new(1),
        auth_mode: auth_mode.clone(),
        jwt_secret,
        issued_tokens: Mutex::new(HashMap::new()),
        sessions: RwLock::new(HashMap::new()),
        rl: Mutex::new(HashMap::new()),
        proxy_rl: Mutex::new(HashMap::new()),
        allow_token_query: args.allow_token_query,
        internal_key: args.internal_key,
        metrics: Metrics::new(),
    });

    let app = tunly::create_app(state.clone());

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
                    tracing::info!("GC: removed {} expired token(s)", removed);
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
                let entries: Vec<(String, Arc<tunly::SessionState>)> = {
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
                        tracing::info!("GC: removed {} stale session(s)", removed);
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

    if args.acme_domains.is_empty() {
        tracing::info!("Tunly Server listening on http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let svc = app.into_make_service_with_connect_info::<SocketAddr>();
        axum::serve(listener, svc).await.unwrap();
    } else {
        tracing::info!(
            "Tunly Server listening with automatic TLS for {}",
            args.acme_domains.join(", ")
        );
        let mut acme_state = AcmeConfig::new(args.acme_domains)
            .contact(
                args.acme_emails
                    .iter()
                    .map(|email| format!("mailto:{email}")),
            )
            .cache_option(Some(DirCache::new(args.acme_cache)))
            .directory_lets_encrypt(!args.acme_staging)
            .state();
        let acceptor = acme_state.axum_acceptor(acme_state.default_rustls_config());

        tokio::spawn(async move {
            while let Some(event) = acme_state.next().await {
                match event {
                    Ok(event) => tracing::info!("ACME event: {:?}", event),
                    Err(error) => tracing::error!("ACME error: {:?}", error),
                }
            }
        });

        axum_server::bind(addr)
            .acceptor(acceptor)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    }
}
