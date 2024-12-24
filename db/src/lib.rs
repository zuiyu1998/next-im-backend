use std::sync::Arc;

use abi::{config::Config, sea_orm::Database, Result};

pub mod database;

mod msg;

use database::MsgDb;
use migration::{Migrator, MigratorTrait};
use msg::MessageStoreRepo;

pub struct DbRepo {
    msg: Arc<dyn MessageStoreRepo>,
}

impl DbRepo {
    pub async fn new(config: &Config) -> Result<DbRepo> {
        let connect = Database::connect(&config.db.databse_url).await?;

        Migrator::up(&connect, None).await?;

        let msg = MsgDb::new(connect);

        let db = DbRepo { msg: Arc::new(msg) };

        Ok(db)
    }
}
