use crate::{
    pb::message::{ChatMsg, ChatMsgType, ChatType},
    utils::get_now,
};
use uuid::Uuid;

pub struct ChatMsgBuilder {
    chat_type: ChatType,
    local_id: Uuid,
    server_id: Uuid,
    server_at: i64,
    seq_id: i64,
    create_at: i64,
    sender_id: i64,
    receiver_id: i64,
    msg_type: ChatMsgType,
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
            create_at: now.timestamp_millis(),
            sender_id: 0,
            receiver_id: 0,
            msg_type: ChatMsgType::Text,
            content: None,
        }
    }
}

impl ChatMsgBuilder {
    pub fn build(self) -> ChatMsg {
        assert_eq!(self.content.is_some(), true);

        ChatMsg {
            local_id: self.local_id.to_string(),
            server_id: self.server_id.to_string(),
            server_at: self.server_at,
            seq_id: self.seq_id,
            create_at: self.create_at,
            sender_id: self.sender_id,
            receiver_id: self.receiver_id,
            msg_type: self.msg_type.into(),
            content: self.content.unwrap(),
            chat_type: self.chat_type.into(),
        }
    }
}
