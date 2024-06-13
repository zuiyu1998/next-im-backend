use serde::{Deserialize, Serialize};
use tonic::async_trait;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    pub service_center: ServiceCenterConfig,

    pub rpc: RpcConfig,

    pub kafka: KafkaConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KafkaConfig {
    pub hosts: Vec<String>,
    pub topic: String,
    pub connect_timeout: u16,
    pub group: String,
    pub producer: KafkaProducer,
    pub consumer: KafkaConsumer,
}

impl Default for KafkaConfig {
    fn default() -> Self {
        KafkaConfig {
            hosts: vec!["192.168.0.103:9092".to_string()],
            topic: "next-chat".to_string(),
            connect_timeout: 5000,
            group: "chat".to_string(),
            producer: KafkaProducer::default(),
            consumer: KafkaConsumer::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KafkaConsumer {
    pub session_timeout: u16,
    pub auto_offset_reset: String,
}

impl Default for KafkaConsumer {
    fn default() -> Self {
        KafkaConsumer {
            session_timeout: 20000,
            auto_offset_reset: "earliest".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KafkaProducer {
    pub timeout: u16,
    pub acks: String,
    pub max_retry: u8,
    pub retry_interval: u16,
}

impl Default for KafkaProducer {
    fn default() -> Self {
        KafkaProducer {
            timeout: 3000,
            acks: "all".to_string(),
            max_retry: 3,
            retry_interval: 1000,
        }
    }
}

//注册中心配置
#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct ServiceCenterConfig {
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub timeout: u64,
}

impl Default for ServiceCenterConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_owned(),
            port: 8500,
            protocol: "http".to_owned(),
            timeout: 5000,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcConfig {
    pub health_check: bool,
    pub msg: RpcServerConfig,
    pub chat: RpcServerConfig,
    // pub db: RpcServerConfig,
    // pub pusher: RpcServerConfig,
}

impl RpcServerConfig {
    #[inline]
    pub fn rpc_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    #[inline]
    pub fn url(&self, https: bool) -> String {
        url(https, &self.host, self.port)
    }
}

fn url(https: bool, host: &str, port: u16) -> String {
    if https {
        format!("https://{}:{}", host, port)
    } else {
        format!("http://{}:{}", host, port)
    }
}

impl Default for RpcConfig {
    fn default() -> Self {
        RpcConfig {
            health_check: false,
            chat: RpcServerConfig {
                protocol: "http".to_owned(),
                port: 50003,
                host: "127.0.0.1".to_owned(),
                name: "chat".to_owned(),
                tags: vec!["chat".to_owned(), "grpc".to_owned()],
                grpc_health_check: GrpcHealthCheck {
                    grpc_use_tls: false,
                    interval: 3000,
                },
            },
            msg: RpcServerConfig {
                protocol: "http".to_owned(),
                port: 50002,
                host: "127.0.0.1".to_owned(),
                name: "msg".to_owned(),
                tags: vec!["msg".to_owned(), "grpc".to_owned()],
                grpc_health_check: GrpcHealthCheck {
                    grpc_use_tls: false,
                    interval: 3000,
                },
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcServerConfig {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub tags: Vec<String>,
    pub grpc_health_check: GrpcHealthCheck,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrpcHealthCheck {
    pub grpc_use_tls: bool,
    pub interval: u16,
}

#[async_trait]
pub trait FromConfig: Sized {
    type Error;

    async fn from_config(config: &Config) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceType {
    Chat,
    Msg,
    All,
}
