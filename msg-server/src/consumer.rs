use std::sync::Arc;

use crate::{
    pusher::{push_service, Pusher},
    Error, Result,
};
use abi::{
    config::Config,
    futures,
    pb::message::{ChatMsg, ChatType},
    serde_json, tokio, tracing,
};
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    ClientConfig, Message,
};

pub struct ConsumerService {
    consumer: StreamConsumer,
    pusher: Arc<dyn Pusher>,
}

impl ConsumerService {
    pub async fn new(config: &Config) -> Self {
        tracing::info!("start kafka consumer:\t{:?}", config.kafka);

        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", &config.kafka.group)
            .set("bootstrap.servers", config.kafka.hosts.join(","))
            .set("enable.auto.commit", "false")
            .set(
                "session.timeout.ms",
                config.kafka.consumer.session_timeout.to_string(),
            )
            .set(
                "socket.timeout.ms",
                config.kafka.connect_timeout.to_string(),
            )
            .set("enable.partition.eof", "false")
            .set(
                "auto.offset.reset",
                config.kafka.consumer.auto_offset_reset.clone(),
            )
            .create()
            .expect("Consumer creation failed");

        consumer
            .subscribe(&[&config.kafka.topic])
            .expect("Can't subscribe to specified topic");

        let pusher = push_service(config).await;

        ConsumerService { consumer, pusher }
    }

    pub async fn consume(&mut self) -> Result<()> {
        loop {
            match self.consumer.recv().await {
                Err(e) => {
                    tracing::error!("Kafka error: {}", e)
                }
                Ok(m) => {
                    if let Some(Ok(payload)) = m.payload_view::<str>() {
                        if let Err(e) = self.handle_msg(payload).await {
                            tracing::error!("Failed to handle message: {:?}", e);
                            continue;
                        }
                        if let Err(e) = self.consumer.commit_message(&m, CommitMode::Async) {
                            tracing::error!("Failed to commit message: {:?}", e);
                        }
                    }
                }
            }
        }
    }

    pub async fn handle_msg(&self, payload: &str) -> Result<()> {
        tracing::debug!("Received message: {:#?}", payload);

        let msg: ChatMsg = serde_json::from_str(payload)?;

        let msg_type: ChatType = ChatType::try_from(msg.chat_type).unwrap();

        let mut tasks = Vec::with_capacity(2);

        let pusher = self.pusher.clone();
        let to_pusher = tokio::spawn(async move {
            match msg_type {
                ChatType::User => {
                    if let Err(e) = pusher.push_single_msg(msg).await {
                        tracing::error!("failed to send message to pusher, error: {:?}", e);
                    }
                }
                ChatType::Group => {
                    tracing::debug!("send message to pusher, msg: {:?}", msg);
                }
            }
        });
        tasks.push(to_pusher);

        futures::future::try_join_all(tasks)
            .await
            .map_err(|e| Error::JoinError(e.to_string()))?;

        Ok(())
    }
}
