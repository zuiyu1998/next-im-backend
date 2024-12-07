mod redis_impl;

use std::sync::Arc;
use abi::{Result, tonic::async_trait, pb::message::Sequence};

#[async_trait]
pub trait Cache: 'static + Send + Sync {
    async fn get_seq(&self, sequence: &Sequence) -> Result<i64>;
    async fn increase_seq(&self, sequence: &Sequence) -> Result<i64>;
}

pub fn get_cache() -> Arc<dyn Cache> {
    todo!()
}


