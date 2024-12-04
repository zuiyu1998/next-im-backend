use abi::{pb::message::Sequence, tonic::async_trait};

use crate::Result;

#[async_trait]
pub trait StoreSequence: Sync + Send {
    //读取序列号
    async fn read_sequence_id(&self, sequence: &Sequence) -> Result<i64>;
    //存储序列号
    async fn store_sequence_id(&self, sequence: &Sequence, id: i64) -> Result<i32>;
}
