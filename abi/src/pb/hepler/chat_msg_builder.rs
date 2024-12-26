use crate::{
    pb::message::{ChatContentType, ChatMsg, ChatType},
    utils::get_now,
};
use serde::Serialize;
use uuid::Uuid;

pub struct ChatMsgBuilder {
    chat_type: ChatType,
    local_id: Uuid,
    server_id: Uuid,
    server_at: i64,
    seq_id: i64,
    local_at: i64,
    sender_id: i64,
    receiver_id: i64,
    content_type: ChatContentType,
    content: Option<Vec<u8>>,
}

impl Default for ChatMsgBuilder {
    fn default() -> Self {
        let now = get_now();

        ChatMsgBuilder {
            chat_type: ChatType::User,
            local_id: Uuid::new_v4(),
            server_id: Uuid::nil(),
            server_at: 0,
            seq_id: 0,
            local_at: now.timestamp_millis(),
            sender_id: 0,
            receiver_id: 0,
            content_type: ChatContentType::Text,
            content: None,
        }
    }
}

impl ChatMsgBuilder {
    pub fn set_content<T: Serialize>(&mut self, v: &T) -> &mut ChatMsgBuilder {
        let value = bincode::serialize(v).unwrap();
        self.content = Some(value);

        self
    }

    pub fn build(self) -> ChatMsg {
        assert_eq!(self.content.is_some(), true);

        ChatMsg {
            local_id: self.local_id.to_string(),
            server_id: self.server_id.to_string(),
            server_at: self.server_at,
            seq_id: self.seq_id,
            local_at: self.local_at,
            sender_id: self.sender_id,
            receiver_id: self.receiver_id,
            content_type: self.content_type.into(),
            content: self.content.unwrap(),
            chat_type: self.chat_type.into(),
        }
    }
}
