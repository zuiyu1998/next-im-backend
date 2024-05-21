use abi::{pb::message::Platfrom, tokio, tracing::Level, tracing_subscriber};

use client::{Client, ClientOptions};

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string())
        .parse()
        .unwrap();

    let mut client = Client::new(ClientOptions { addr });

    client.connect("lw", "123456", Platfrom::Windows).await?;

    client.run().await;

    Ok(())
}
