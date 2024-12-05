use abi::{
    config::Config,
    tokio,
    tracing::{self, Level},
    tracing_subscriber,
};
use db::DbRpcService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let config = Config::default();
    if let Err(e) = DbRpcService::start(&config).await {
        tracing::error!("db rpc start error: {}", e);
    }
}
