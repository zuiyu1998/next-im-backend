use abi::{
    tokio::{self},
    tracing_subscriber,
};

use client::{Client, ClientOptions};

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string())
        .parse()
        .unwrap();

    let client = Client::new(ClientOptions { addr });

    client.run().await;

    Ok(())
}
