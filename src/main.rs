use std::net::SocketAddr;

use clap::Parser;
use rust_template::router;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// HTTP listen address
    #[arg(long, env = "LISTEN", default_value = "0.0.0.0:8080")]
    listen: SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer())
        .init();

    let cli = Cli::parse();
    let listener = TcpListener::bind(cli.listen).await?;
    info!(addr = %cli.listen, "listening");
    axum::serve(listener, router()).await?;
    Ok(())
}
