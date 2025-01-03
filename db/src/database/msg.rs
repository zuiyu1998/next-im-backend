use abi::{
    pb::message::{ChatContentType, ChatMsg, ChatType},
    sea_orm::{ActiveModelTrait, DatabaseConnection, Set},
    tonic::async_trait,
    Result,
};
use entity::msg::MsgActiveModel;

use crate::msg::MessageStoreRepo;

#[derive(Debug)]
pub struct MsgDb {
    pub conn: DatabaseConnection,
}

impl MsgDb {
    pub fn new(conn: DatabaseConnection) -> MsgDb {
        MsgDb { conn }
    }
}

#[async_trait]
impl MessageStoreRepo for MsgDb {
    async fn save_message(&self, chat_msg: ChatMsg) -> Result<()> {
        let mut active: MsgActiveModel = <MsgActiveModel as ActiveModelTrait>::default();
        active.local_id = Set(chat_msg.local_id);
        active.server_id = Set(chat_msg.server_id);
        active.server_at = Set(chat_msg.server_at);
        active.seq_id = Set(chat_msg.seq_id);
        active.local_at = Set(chat_msg.local_at);
        active.sender_id = Set(chat_msg.sender_id);
        active.receiver_id = Set(chat_msg.receiver_id);
        active.content_type = Set(ChatContentType::try_from(chat_msg.content_type)
            .unwrap()
            .as_str_name()
            .to_owned());
        active.content = Set(chat_msg.content);
        active.chat_type = Set(ChatType::try_from(chat_msg.chat_type)
            .unwrap()
            .as_str_name()
            .to_owned());

        active.insert(&self.conn).await?;

        Ok(())
    }
}
