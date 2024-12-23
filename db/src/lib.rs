use abi::{config::Config, sea_orm::Database, Result};
use std::sync::Arc;

pub mod database;
pub mod seq;

use database::SeqDb;
use migration::{Migrator, MigratorTrait};
use seq::SeqRepo;

pub struct DbRepo {
    _seq: Arc<dyn SeqRepo>,
}

impl DbRepo {
    pub async fn new(config: &Config) -> Result<DbRepo> {
        let connect = Database::connect(&config.db.databse_url).await?;

        Migrator::up(&connect, None).await?;

        let seq = Arc::new(SeqDb {
            conn: connect.clone(),
        });

        let db = DbRepo { _seq: seq };

        Ok(db)
    }
}
