use serde::{Deserialize, Serialize};
use tonic::async_trait;
use tracing::Level;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub enum TraceLevel {
    #[default]
    Debug,
}

impl TraceLevel {
    pub fn level(&self) -> Level {
        match self {
            TraceLevel::Debug => Level::DEBUG,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LogConfig {
    pub level: TraceLevel,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    pub service_center: ServiceCenterConfig,
    pub rpc: RpcConfig,
    pub kafka: KafkaConfig,
    pub db: DbConfig,
    pub redis: RedisConfig,
    pub api: ApiConfig,
    pub log: LogConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            host: "127.0.0.1".to_owned(),
            port: 6143,
        }
    }
}

impl ApiConfig {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub seq_step: i32,
}

impl Default for RedisConfig {
    fn default() -> Self {
        RedisConfig {
            host: "127.0.0.1".to_string(),
            port: 6379,
            seq_step: 10000,
        }
    }
}

impl RedisConfig {
    pub fn url(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbConfig {
    pub databse_url: String,
}

impl Default for DbConfig {
    fn default() -> Self {
        DbConfig {
            databse_url: "postgresql://postgres:bj123456@localhost/next-im-backend".to_string(),
        }
    }
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
    pub ip: String,
    pub port: u16,
    pub teant: String,
}

impl ServiceCenterConfig {
    pub fn endpoint_addrs(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl Default for ServiceCenterConfig {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_owned(),
            port: 8848,
            teant: "next-im".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcConfig {
    pub msg: RpcServerConfig,
    pub chat: RpcServerConfig,
    pub db: RpcServerConfig,
    // pub pusher: RpcServerConfig,
}

impl RpcServerConfig {
    #[inline]
    pub fn rpc_server_url(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl Default for RpcConfig {
    fn default() -> Self {
        RpcConfig {
            chat: RpcServerConfig {
                port: 50003,
                ip: "127.0.0.1".to_owned(),
                service_name: "chat".to_owned(),
                group_name: "chat-group".to_owned(),
            },
            msg: RpcServerConfig {
                port: 50002,
                ip: "127.0.0.1".to_owned(),
                service_name: "msg".to_owned(),
                group_name: "msg-group".to_owned(),
            },
            db: RpcServerConfig {
                port: 50004,
                ip: "127.0.0.1".to_owned(),
                service_name: "db".to_owned(),
                group_name: "db-group".to_owned(),
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcServerConfig {
    pub ip: String,
    pub port: u32,
    pub service_name: String,
    pub group_name: String,
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
    Db,
}

impl ServiceType {
    pub fn get_rpc_config(&self, config: &Config) -> RpcServerConfig {
        let rpc_config: RpcServerConfig = match self {
            ServiceType::Chat => config.rpc.chat.clone(),

            ServiceType::Msg => config.rpc.msg.clone(),
            ServiceType::Db => config.rpc.db.clone(),

            ServiceType::All => todo!("ALL"),
        };

        rpc_config
    }
}
