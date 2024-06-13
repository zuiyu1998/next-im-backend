mod error;
mod rpc;
mod session;
pub mod store;

use std::sync::Arc;

use abi::{dashmap::DashMap, pb::message::Sequence};
pub use error::*;

use session::Session;
use store::StoreSequence;

pub struct SeqRpcService {
    store: Arc<dyn StoreSequence>,
    sequence_map: DashMap<Sequence, Session>,
    session_width: i64,
}

impl SeqRpcService {
    pub async fn get_id(&self, sequence: Sequence) -> Result<i64> {
        if let Some(mut session) = self.sequence_map.get_mut(&sequence) {
            let seq_id = session.get_sequence_id(&self.store).await?;

            return Ok(seq_id);
        } else {
            let mut session = Session::new(self.session_width);

            let seq_id = session.get_sequence_id(&self.store).await?;

            self.sequence_map.insert(sequence, session);

            return Ok(seq_id);
        }
    }
}
