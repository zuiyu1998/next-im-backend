use std::collections::HashMap;

use crate::{Error, Result};
use abi::{reqwest::Client, tonic::async_trait, tracing};

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

impl Consul {
    pub fn api_url(&self, name: &str) -> String {
        self.url("agent", name)
    }

    fn url(&self, type_: &str, name: &str) -> String {
        format!(
            "{}://{}:{}/v1/{}/{}",
            self.options.protocol, self.options.host, self.options.port, type_, name
        )
    }
}

#[async_trait]
impl ServiceRegister for Consul {
    async fn register(&self, registration: Registration) -> Result<()> {
        let url = self.api_url("service/register");
        let response = self.client.put(&url).json(&registration).send().await?;
        tracing::debug!("register service: {:?} to consul{url}", registration);
        if !response.status().is_success() {
            return Err(Error::InternalServer(
                response.text().await.unwrap_or_default(),
            ));
        }
        Ok(())
    }

    /// 服务发现
    async fn discovery(&self) -> Result<Services> {
        let url = self.api_url("services");
        let services = self
            .client
            .get(url)
            .send()
            .await?
            .json::<Services>()
            .await?;
        Ok(services)
    }

    /// 服务注销
    async fn deregister(&self, service_id: &str) -> Result<()> {
        let url = self.api_url(&format!("service/deregister/{}", service_id));
        let response = self.client.put(url).send().await?;
        if !response.status().is_success() {
            return Err(Error::InternalServer(
                response.text().await.unwrap_or_default(),
            ));
        }
        Ok(())
    }

    /// 服务筛选
    async fn filter_by_name(&self, name: &str) -> Result<Services> {
        let url = self.api_url("services");
        let mut map = HashMap::new();
        map.insert("filter", format!("Service == {}", name));

        let services = self
            .client
            .get(url)
            .query(&map)
            .send()
            .await?
            .json::<Services>()
            .await?;
        Ok(services)
    }
}
