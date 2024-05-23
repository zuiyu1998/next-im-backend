use crate::Result;
use abi::{reqwest::Client, tonic::async_trait};

use super::{Registration, ServiceRegister, Services};

#[derive(Debug, Clone)]
pub struct Consul {
    pub options: ConsulOptions,
    pub client: Client,
}

#[derive(Debug, Clone)]
pub struct ConsulOptions {
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub timeout: u64,
}

#[async_trait]
impl ServiceRegister for Consul {
    async fn register(&self, registration: Registration) -> Result<()> {
        todo!()
    }

    /// 服务发现
    async fn discovery(&self) -> Result<Services> {
        todo!()
    }

    /// 服务注销
    async fn deregister(&self, service_id: &str) -> Result<()> {
        todo!()
    }

    /// 服务筛选
    async fn filter_by_name(&self, name: &str) -> Result<Services> {
        todo!()
    }
}
