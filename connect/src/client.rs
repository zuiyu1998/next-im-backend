use std::{collections::HashMap, sync::Arc};

use abi::{pb::message::Platfrom, stream::MessageStream, UserId};

pub struct Client {
    pub user_id: UserId,
    pub streams: HashMap<Platfrom, Arc<dyn MessageStream>>,
}
