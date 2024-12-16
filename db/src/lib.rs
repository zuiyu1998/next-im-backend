use abi::{
    config::{Config, ServiceType},
    pb::message::db_service_server::DbServiceServer,
    sea_orm::Database,
    tonic::transport::Server,
    tracing,
    utils::register_service,
    Result,
};
use std::sync::Arc;

pub mod database;
pub mod seq;

mod rpc;

use database::SeqDb;
use migration::{Migrator, MigratorTrait};
use seq::SeqRepo;

pub struct DbRpcService {
    seq: Arc<dyn SeqRepo>,
}

impl DbRpcService {
    pub async fn start(config: &Config) -> Result<()> {
        register_service(config, ServiceType::Chat);
        tracing::info!("<db> rpc service register to service register center");

        let connect = Database::connect(&config.db.databse_url).await?;

        Migrator::up(&connect, None).await?;

        let seq = Arc::new(SeqDb {
            conn: connect.clone(),
        });

        let db_rpc = DbRpcService { seq };

        let service = DbServiceServer::new(db_rpc);
        tracing::info!(
            "<db> rpc service started at {}",
            config.rpc.db.rpc_server_url()
        );

        Server::builder()
            .add_service(service)
            .serve(config.rpc.chat.rpc_server_url().parse().unwrap())
            .await
            .unwrap();

        Ok(())
    }
}
