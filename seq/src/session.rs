use std::sync::Arc;

use crate::{store::StoreSequence, Result};

pub struct Session {
    pub seq_id: i64,
    //同时并发数
    width: i64,
    next_max_seq_id: i64,
}

impl Session {
    pub async fn get_sequence_id(&mut self, store: &Arc<dyn StoreSequence>) -> Result<i64> {
        let next_seq_id = self.seq_id + 1;

        if next_seq_id >= self.next_max_seq_id {
            let next_max_seq_id = self.next_max_seq_id(next_seq_id);

            store.store_sequence_id(next_max_seq_id).await?;

            self.next_max_seq_id = next_max_seq_id;
        }

        self.seq_id = next_seq_id;

        Ok(next_seq_id)
    }

    pub fn new(width: i64) -> Self {
        Self {
            seq_id: 0,
            width,
            next_max_seq_id: 1,
        }
    }

    fn next_max_seq_id(&self, seq_id: i64) -> i64 {
        self.width + seq_id
    }
}
