use abi::{config::Config, tokio, tracing::Level, tracing_subscriber};
use chat::ChatRpcService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let config = Config::default();
    ChatRpcService::start(&config).await;
}
