mod tonic_service_discovery;

use std::{collections::HashSet, net::SocketAddr};

use crate::Result;
use abi::tonic::async_trait;

pub use tonic_service_discovery::*;

#[async_trait]
pub trait ServiceFetcher: Send + Sync {
    async fn fetch(&self) -> Result<HashSet<SocketAddr>>;
}
