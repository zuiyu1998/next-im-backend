use abi::{
    config::Config,
    log,
    tokio::{self},
};
use msg_gateway::tcp::TcpServer;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::default();

    log::tracing_subscriber_init(&config);

    TcpServer::start(config).await?;

    Ok(())
}
