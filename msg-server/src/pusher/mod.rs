pub mod service;

use std::{fmt::Debug, sync::Arc};

use crate::Result;
use abi::{config::Config, pb::message::ChatMsg, tonic::async_trait};

#[async_trait]
pub trait Pusher: Send + Sync + Debug {
    async fn push_single_msg(&self, msg: ChatMsg) -> Result<()>;
}


pub async fn push_service(config: &Config) -> Arc<dyn Pusher> {
    Arc::new(service::PusherService::new(config))
}