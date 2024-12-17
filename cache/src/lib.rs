mod redis_impl;

use abi::{config::Config, pb::message::Sequence, tonic::async_trait, Result};
use redis_impl::RedisCache;
use std::sync::Arc;

#[async_trait]
pub trait Cache: 'static + Send + Sync {
    async fn get_seq(&self, sequence: &Sequence) -> Result<i64>;
    async fn increase_seq(&self, sequence: &Sequence) -> Result<i64>;
}

pub fn get_cache(config: &Config) -> Arc<dyn Cache> {
    Arc::new(RedisCache::from_config(config))
}
