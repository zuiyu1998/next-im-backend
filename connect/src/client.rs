use std::collections::HashMap;

use abi::{pb::message::Platfrom, stream::MessageStream, UserId};

pub struct Client {
    pub user_id: UserId,
    pub streams: HashMap<Platfrom, Box<dyn MessageStream>>,
}
