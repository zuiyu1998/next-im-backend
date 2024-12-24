pub mod consumer;
pub mod error;
pub mod producer;
pub mod pusher;

use consumer::ConsumerService;
pub use error::*;

use abi::{config::Config, tokio};
use producer::ChatProducerRpcService;

pub async fn start(config: &Config) {
    let cloned_conf = config.clone();
    let producer = tokio::spawn(async move {
        ChatProducerRpcService::start(&cloned_conf).await;
    });

    let cloned_conf = config.clone();
    let consumer = tokio::spawn(async move {
        ConsumerService::new(&cloned_conf)
            .await
            .consume()
            .await
            .unwrap();
    });

    tokio::try_join!(producer, consumer).unwrap();
}
