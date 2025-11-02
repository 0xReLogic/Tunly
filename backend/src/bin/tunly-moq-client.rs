use clap::Parser;
use tunly::moq::transport::MoqTransport;
use tunly::moq::bridge::RequestEnvelope;
use tunly::moq::transport::{MoqTailTransport, NullMoq};

#[derive(Parser, Debug, Clone)]
#[command(name = "tunly-moq-client", about = "Tunly MoQ Client POC")]
struct Args {
    #[arg(long)]
    remote: Option<String>,
    #[arg(long)]
    token: Option<String>,
    #[arg(long, default_value = "127.0.0.1:8080")]
    local: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();

    // Only connect to MoQ when remote is provided; otherwise keep as no-op
    if let Some(ref r) = args.remote {
        let mut transport = MoqTailTransport::new()?;
        transport.connect(r, args.token.clone()).await?;
        let _ = transport; // placeholder until real HTTP bridge wiring
    } else {
        let _noop = NullMoq::new();
        let _ = _noop;
    }

    let _sample = RequestEnvelope {
        id: 1,
        method: "GET".to_string(),
        uri: "/".to_string(),
        headers: vec![],
        body: vec![],
    };

    println!("tunly-moq-client POC initialized");
    Ok(())
}
