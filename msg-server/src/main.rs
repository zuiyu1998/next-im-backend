use abi::{
    config::Config,
    log,
    tokio::{self},
};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::default();

    log::tracing_subscriber_init(&config);

    msg_server::start(&config).await;

    Ok(())
}
