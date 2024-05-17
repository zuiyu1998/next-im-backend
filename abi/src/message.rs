use crate::pb::message::{msg::Union, Msg};

pub type UserId = i64;

pub trait Message {
    fn get_sender_id(&self) -> Option<UserId>;
}

impl Message for Msg {
    fn get_sender_id(&self) -> Option<UserId> {
        if self.union.is_none() {
            return None;
        }

        (&self.union).as_ref().and_then(|union| match union {
            Union::ChatMsg(msg) => Some(msg.sender_id),
            // _ => None,
        })
    }
}
