use crate::{service_discovery::ServiceFetcher, service_register_center::ServiceRegister, Result};
use abi::{tonic::async_trait, tracing::warn};
use std::{collections::HashSet, net::SocketAddr, sync::Arc};

pub struct ServiceResolver {
    service_name: String,
    service_center: Arc<dyn ServiceRegister>,
}

#[async_trait]
impl ServiceFetcher for ServiceResolver {
    async fn fetch(&self) -> Result<HashSet<SocketAddr>> {
        let map = self
            .service_center
            .filter_by_name(&self.service_name)
            .await?;
        let x = map
            .values()
            .filter_map(|v| match format!("{}:{}", v.address, v.port).parse() {
                Ok(s) => Some(s),
                Err(e) => {
                    warn!("parse address error:{}", e);
                    None
                }
            })
            .collect();
        Ok(x)
    }
}

impl ServiceResolver {
    pub fn new(service_center: Arc<dyn ServiceRegister>, service_name: String) -> Self {
        Self {
            service_name,
            service_center,
        }
    }
}
