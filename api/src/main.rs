use abi::config::Config;
use abi::{log, tokio, tracing};

use abi::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = Config::default();

    log::tracing_subscriber_init(&config);

    if let Err(e) = api::start(&config).await {
        tracing::error!("start error: {}", e);
    }

    Ok(())
}
