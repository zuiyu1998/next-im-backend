use abi::utils::register_service;
use abi::{
    chrono,
    config::{Config, ServiceType},
    nanoid::nanoid,
    pb::message::{
        chat_service_server::{ChatService, ChatServiceServer},
        ChatMsg, MsgResponse,
    },
    tonic::{async_trait, transport::Server, Request, Response, Status},
    tracing,
};
use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    client::DefaultClientContext,
    error::KafkaError,
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};

use std::time::Duration;

pub struct ChatRpcService {
    kafka: FutureProducer,
    topic: String,
}

impl ChatRpcService {
    pub fn new(kafka: FutureProducer, topic: String) -> Self {
        Self { kafka, topic }
    }

    pub async fn start(config: &Config) {
        let broker = config.kafka.hosts.join(",");

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &broker)
            .set(
                "message.timeout.ms",
                config.kafka.producer.timeout.to_string(),
            )
            .set(
                "socket.timeout.ms",
                config.kafka.connect_timeout.to_string(),
            )
            .set("acks", config.kafka.producer.acks.clone())
            // make sure the message is sent exactly once
            .set("enable.idempotence", "true")
            .set("retries", config.kafka.producer.max_retry.to_string())
            .set(
                "retry.backoff.ms",
                config.kafka.producer.retry_interval.to_string(),
            )
            .create()
            .expect("Producer creation error");

        Self::ensure_topic_exists(&config.kafka.topic, &broker, config.kafka.connect_timeout)
            .await
            .expect("Topic creation error");

        register_service(config, ServiceType::Chat);
        tracing::info!("<chat> rpc service register to service register center");

        let chat_rpc = Self::new(producer, config.kafka.topic.clone());
        let service = ChatServiceServer::new(chat_rpc);
        tracing::info!(
            "<chat> rpc service started at {}",
            config.rpc.chat.rpc_server_url()
        );

        Server::builder()
            .add_service(service)
            .serve(config.rpc.chat.rpc_server_url().parse().unwrap())
            .await
            .unwrap();
    }

    async fn ensure_topic_exists(
        topic_name: &str,
        brokers: &str,
        timeout: u16,
    ) -> Result<(), KafkaError> {
        // Create Kafka AdminClient
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("socket.timeout.ms", timeout.to_string())
            .create()?;

        // create topic
        let new_topics = [NewTopic {
            name: topic_name,
            num_partitions: 1,
            replication: TopicReplication::Fixed(1),
            config: vec![],
        }];

        // fixme not find the way to check topic exist
        // so just create it and judge the error,
        // but don't find the error type for topic exist
        // and this way below can work well.
        let options = AdminOptions::new();
        admin_client.create_topics(&new_topics, &options).await?;
        match admin_client.create_topics(&new_topics, &options).await {
            Ok(_) => {
                tracing::info!("Topic not exist; create '{}' ", topic_name);
                Ok(())
            }
            Err(KafkaError::AdminOpCreation(_)) => {
                println!("Topic '{}' already exists.", topic_name);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

#[async_trait]
impl ChatService for ChatRpcService {
    async fn send_message(
        &self,
        request: Request<ChatMsg>,
    ) -> Result<Response<MsgResponse>, Status> {
        let mut msg = request.into_inner();

        msg.server_id = nanoid!();

        msg.server_at = chrono::Local::now()
            .naive_local()
            .and_utc()
            .timestamp_millis();

        // send msg to kafka
        let payload = serde_json::to_string(&msg).unwrap();
        // let kafka generate key, then we need set FutureRecord<String, type>
        let record: FutureRecord<String, String> = FutureRecord::to(&self.topic).payload(&payload);

        tracing::info!("send msg to kafka: {:?}", record);

        let err = match self.kafka.send(record, Duration::from_secs(0)).await {
            Ok(_) => String::new(),
            Err((err, msg)) => {
                tracing::error!(
                    "send msg to kafka error: {:?}; owned message: {:?}",
                    err,
                    msg
                );
                err.to_string()
            }
        };

        return Ok(Response::new(MsgResponse {
            local_id: msg.local_id,
            server_id: msg.server_id,
            server_at: msg.server_at,
            err,
        }));
    }
}
