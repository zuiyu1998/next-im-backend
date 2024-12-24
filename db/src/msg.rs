use abi::{pb::message::ChatMsg, tonic::async_trait, Result};

#[async_trait]
pub trait MessageStoreRepo {
   async fn save_message(&self, chat_msg: ChatMsg) -> Result<()>;
}