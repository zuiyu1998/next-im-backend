use abi::tonic::async_trait;

use crate::Result;

#[async_trait]
pub trait StoreSequence: Sync + Send {
    //读取序列号
    async fn read_sequence_id(&self) -> Result<i64>;
    //存储序列号
    async fn store_sequence_id(&self, id: i64) -> Result<i32>;
}
