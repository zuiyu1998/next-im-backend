mod redis_impl;

use abi::{config::Config, pb::message::Sequence, tonic::async_trait, Result, UserId};
use redis_impl::RedisCache;
use std::sync::Arc;

#[async_trait]
pub trait Cache: 'static + Send + Sync {
    async fn get_seq(&self, sequence: &Sequence) -> Result<i64>;
    async fn increase_seq(&self, sequence: &Sequence) -> Result<i64>;

    //设置user token
    async fn set_user_token(&self, user_id: UserId, token: Option<String>) -> Result<()>;
    //获取user token
    async fn get_user_token(&self, user_id: UserId) -> Result<Option<String>>;
}

pub fn get_cache(config: &Config) -> Arc<dyn Cache> {
    Arc::new(RedisCache::from_config(config))
}
