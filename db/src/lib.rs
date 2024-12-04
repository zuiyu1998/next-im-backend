use std::sync::Arc;
use abi::Result;

pub mod database;
pub mod seq;

mod rpc;

use seq::SeqRepo;

pub struct DbRpcService {
    seq: Arc<dyn SeqRepo>,
}

impl DbRpcService {
    pub async fn start() -> Result<()> {
        Ok(())
    }
}
