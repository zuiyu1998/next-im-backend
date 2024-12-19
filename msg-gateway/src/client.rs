use std::{collections::HashMap, sync::Arc};

use abi::{message::MessageSink, pb::message::Platfrom, tokio::sync::RwLock, UserId};

pub struct Client {
    pub user_id: UserId,
    pub sinks: HashMap<Platfrom, Arc<RwLock<Box<dyn MessageSink>>>>,
}
