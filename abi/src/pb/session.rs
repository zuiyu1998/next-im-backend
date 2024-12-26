use crate::message::Message;

use super::message::{ChatMsg, ChatType};

#[derive(Debug, Clone)]
pub struct Session {
    chat_type: ChatType,
    sender_id: i64,
    receiver_id: i64,
}

impl Session {
    pub fn from_chat_msg(chat_msg: &ChatMsg) -> Self {
        Session {
            chat_type: ChatType::try_from(chat_msg.chat_type).unwrap(),
            sender_id: chat_msg.sender_id,
            receiver_id: chat_msg.receiver_id,
        }
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl ToString for Session {
    fn to_string(&self) -> String {
        match self.chat_type {
            ChatType::User => {
                let max = self.sender_id.max(self.receiver_id);
                let min = self.sender_id.min(self.receiver_id);
                format!("{}:{}-{}", self.chat_type.as_str_name(), max, min)
            }
            ChatType::Group => {
                format!("{}:{}", self.sender_id, self.receiver_id)
            }
        }
    }
}
