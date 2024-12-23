use crate::Result;
use abi::{config::Config, pb::message::Msg, serde_json, tracing};
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    ClientConfig, Message,
};

pub struct ConsumerService {
    consumer: StreamConsumer,
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

        ConsumerService { consumer }
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

        let mut _msg: Msg = serde_json::from_str(payload)?;

        // let mut tasks = Vec::with_capacity(2);

        // futures::future::try_join_all(tasks).await?;

        Ok(())
    }
}
