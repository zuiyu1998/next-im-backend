use abi::config::Config;
use abi::tracing::Level;
use abi::{tokio, tracing, tracing_subscriber};

use abi::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let config: Config = Config::default();

    if let Err(e) = api::start(&config).await {
        tracing::error!("start error: {}", e);
    }

    Ok(())
}
