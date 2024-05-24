use serde::{Deserialize, Serialize};
use tonic::async_trait;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    pub service_center: ServiceCenterConfig,

    pub rpc: RpcConfig,
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
        todo!()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcConfig {
    pub health_check: bool,
    pub msg: RpcServerConfig,
    pub chat: RpcServerConfig,
    pub db: RpcServerConfig,
    pub pusher: RpcServerConfig,
}

impl Default for RpcConfig {
    fn default() -> Self {
        todo!()
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
pub trait FromConfig {
    async fn from_conifg(&self, conifg: &Config) -> Self;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceType {
    Chat,
    Msg,
    All,
}
