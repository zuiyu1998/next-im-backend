use std::collections::HashMap;

use abi::{message::UserId, pb::message::Platfrom};

use crate::stream::MessageStream;

pub struct Client {
    pub user_id: UserId,
    pub streams: HashMap<Platfrom, Box<dyn MessageStream>>,
}
