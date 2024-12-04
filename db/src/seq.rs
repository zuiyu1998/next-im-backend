use std::fmt::Debug;
use abi::{async_trait::async_trait, Result, pb::message::Sequence};

#[async_trait]
pub trait SeqRepo: Sync + Send + Debug {
    //读取序列号
    async fn read_sequence_id(&self, sequence: &Sequence) -> Result<i64>;
    //更新序列号
    async fn update_sequence_id(&self, sequence: &Sequence, id: i64) -> Result<i64>;
    //创建序列号
    async fn create_sequence_id(&self, sequence: &Sequence) -> Result<i64>;
}
