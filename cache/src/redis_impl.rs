use crate::Cache;
use abi::{
    pb::message::Sequence,
    redis::{self, AsyncCommands},
    tonic::async_trait,
    Result,
};

#[derive(Debug)]
pub struct RedisCache {
    client: redis::Client,
}

#[async_trait]
impl Cache for RedisCache {
    async fn get_seq(&self, sequence: &Sequence) -> Result<i64> {
        let key = format!(
            "seq:chart_type-{}-sender_id-{}-receiver_id-{}",
            sequence.chat_type, sequence.sender_id, sequence.receiver_id
        );

        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let seq: i64 = conn.get(&key).await.unwrap_or_default();
        Ok(seq)
    }

    async fn increase_seq(&self, sequence: &Sequence) -> Result<i64> {
        let key = format!(
            "seq:chart_type-{}-sender_id-{}-receiver_id-{}",
            sequence.chat_type, sequence.sender_id, sequence.receiver_id
        );
        
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let seq: i64 = conn.incr(&key, 1).await?;
        Ok(seq)
    }
}
