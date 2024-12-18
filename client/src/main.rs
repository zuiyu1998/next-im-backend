use abi::{config::Config, log::tracing_subscriber_init, tokio};
use client::Client;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::default();

    tracing_subscriber_init(&config);

    let mut client = Client::from_config(&config);

    client.connect(1, "test").await?;

    Ok(())
}
