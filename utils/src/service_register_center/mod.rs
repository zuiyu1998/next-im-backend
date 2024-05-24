pub mod consul;

use crate::Result;
use abi::tonic::async_trait;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

//服务集合
pub type Services = HashMap<String, Service>;

//服务
#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub id: String,
    pub service: String,
    pub address: String,
    pub port: u16,
    pub tags: Vec<String>,
    pub datacenter: String,
}

//注册所需数据
#[derive(Serialize, Deserialize, Debug)]
pub struct Registration {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub tags: Vec<String>,
    pub check: Option<GrpcHealthCheck>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GrpcHealthCheck {
    pub name: String,
    pub grpc: String,
    pub grpc_use_tls: bool,
    pub interval: String,
}

#[async_trait]
pub trait ServiceRegister: Send + Sync + Debug {
    /// 服务注册
    async fn register(&self, registration: Registration) -> Result<()>;

    /// 服务发现
    async fn discovery(&self) -> Result<Services>;

    /// 服务注销
    async fn deregister(&self, service_id: &str) -> Result<()>;

    /// 服务筛选
    async fn filter_by_name(&self, name: &str) -> Result<Services>;
}
