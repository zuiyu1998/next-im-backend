mod redis_impl;

use abi::{config::Config, pb::session::Session, tonic::async_trait, Result, UserId};
use redis_impl::RedisCache;
use std::sync::Arc;

#[async_trait]
pub trait Cache: 'static + Send + Sync {
    //设置user token
    async fn set_user_token(&self, user_id: UserId, token: Option<String>) -> Result<()>;
    //获取user token
    async fn get_user_token(&self, user_id: UserId) -> Result<Option<String>>;

    //获取当前的消息序列号
    async fn get_seq(&self, session: &Session) -> Result<i64>;

    //递增消息序列号
    async fn increase_seq(&self, session: &Session) -> Result<i64>;

}

pub fn get_cache(config: &Config) -> Arc<dyn Cache> {
    Arc::new(RedisCache::from_config(config))
}
