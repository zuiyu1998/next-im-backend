use crate::Cache;
use abi::{
    config::Config,
    pb::session::Session,
    redis::{self, AsyncCommands},
    tonic::async_trait,
    Result, UserId,
};

pub const USE_TOKEN_KEY: &str = "user_token";

#[derive(Debug)]
pub struct RedisCache {
    client: redis::Client,
}

impl RedisCache {
    pub fn from_config(config: &Config) -> RedisCache {
        let client = redis::Client::open(config.redis.url()).expect("redis open fail");
        RedisCache { client }
    }
}

#[async_trait]
impl Cache for RedisCache {
    async fn get_seq(&self, session: &Session) -> Result<i64> {
        let key = session.to_string();

        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let seq: i64 = conn.get(&key).await.unwrap_or_default();
        Ok(seq)
    }

    async fn increase_seq(&self, session: &Session) -> Result<i64> {
        let key = session.to_string();

        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let seq: i64 = conn.incr(&key, 1).await?;
        Ok(seq)
    }

    async fn set_user_token(&self, user_id: UserId, token: Option<String>) -> Result<()> {
        let key = format!("user_token:user_id-{}", user_id);
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        if token.is_none() {
            let _: () = conn.hdel(USE_TOKEN_KEY, &[key]).await?;
            Ok(())
        } else {
            let _: () = conn.hset(USE_TOKEN_KEY, &key, token.unwrap()).await?;
            Ok(())
        }
    }

    async fn get_user_token(&self, user_id: UserId) -> Result<Option<String>> {
        let key = format!("user_token:user_id-{}", user_id);
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let user_token: Option<String> = conn.hget(USE_TOKEN_KEY, &key).await?;
        Ok(user_token)
    }
}
