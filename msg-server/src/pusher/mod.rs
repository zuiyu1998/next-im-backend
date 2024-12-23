pub mod service;

use std::fmt::Debug;

use crate::Result;
use abi::{pb::message::ChatMsg, tonic::async_trait};

#[async_trait]
pub trait Pusher: Send + Sync + Debug {
    async fn push_single_msg(&self, msg: ChatMsg) -> Result<()>;
}
