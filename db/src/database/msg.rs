use abi::{pb::message::ChatMsg, sea_orm::DatabaseConnection, tonic::async_trait, Result};

use crate::msg::MessageStoreRepo;

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
    fn save_message(&self, chat_msg: ChatMsg) -> Result<()> {
        todo!()
    }
}
