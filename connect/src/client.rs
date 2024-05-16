use std::collections::HashMap;

use abi::pb::message::Platfrom;

use crate::stream::MessageStream;

pub type UserId = i64;

pub struct Client {
    pub user_id: UserId,
    pub streams: HashMap<Platfrom, Box<dyn MessageStream>>,
}
