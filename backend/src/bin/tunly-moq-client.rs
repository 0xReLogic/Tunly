use clap::Parser;
use tunly::moq::bridge::RequestEnvelope;
use tunly::moq::transport::{MoqTransport, NullMoq};

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut transport = NullMoq::new();
    if let Some(ref r) = args.remote {
        transport.connect(r, args.token.clone())?;
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
