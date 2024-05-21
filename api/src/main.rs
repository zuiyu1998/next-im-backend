use abi::tracing::Level;
use abi::{tokio, tracing, tracing_subscriber};

use abi::Result;
use api::start;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    if let Err(e) = start().await {
        tracing::error!("start error: {}", e);
    }

    Ok(())
}
