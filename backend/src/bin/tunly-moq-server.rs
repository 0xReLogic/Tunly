use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "tunly-moq-server", about = "Tunly MoQ Server POC")]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    #[arg(long, env = "PORT", default_value_t = 8081)]
    port: u16,
    #[arg(long)]
    token: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let addr = format!("{}:{}", args.host, args.port);
    println!("tunly-moq-server POC listening on {}", addr);
    Ok(())
}
