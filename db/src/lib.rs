use std::sync::Arc;

use database::SeqRepo;

pub mod database;

mod rpc;

pub struct DbRpcService {
    seq: Arc<dyn SeqRepo>,
}
