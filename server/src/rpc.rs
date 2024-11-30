use crate::{manager::Manager, Result};
use abi::{
    config::{Config, ServiceType},
    pb::message::{
        msg_service_server::{MsgService, MsgServiceServer},
        ChatMsg, SendMsgResponse,
    },
    synapse::health::{HealthServer, HealthService},
    tonic::{async_trait, transport::Server, Request, Response, Status},
    tracing,
};

use utils::helpers;

pub struct ConnectRpcService {
    manager: Manager,
}

impl ConnectRpcService {
    pub fn new(manager: Manager) -> Self {
        Self { manager }
    }

    pub async fn start(manager: Manager, config: &Config) -> Result<()> {
        // register service to service register center
        helpers::register_service(config, ServiceType::Msg).await?;
        tracing::info!("<connect> rpc service register to service register center");

        // open health check
        let health_service = HealthServer::new(HealthService::new());
        tracing::info!("<connect> rpc service health check started");

        let service = Self::new(manager);
        let svc = MsgServiceServer::new(service);
        tracing::info!(
            "<connect> rpc service started at {}",
            config.rpc.msg.rpc_server_url()
        );

        Server::builder()
            .add_service(health_service)
            .add_service(svc)
            .serve(config.rpc.msg.rpc_server_url().parse().unwrap())
            .await
            .unwrap();
        Ok(())
    }
}

#[async_trait]
impl MsgService for ConnectRpcService {
    async fn send_message(
        &self,
        request: Request<ChatMsg>,
    ) -> Result<Response<SendMsgResponse>, Status> {
        let msg = request.into_inner();

        self.manager.broadcast(msg).await;
        let response = Response::new(SendMsgResponse {});
        Ok(response)
    }

    async fn send_msg_to_user(
        &self,
        request: Request<ChatMsg>,
    ) -> Result<Response<SendMsgResponse>, Status> {
        let msg = request.into_inner();

        self.manager.send_single_msg(&msg.sender_id, &msg).await;

        let response = Response::new(SendMsgResponse {});
        Ok(response)
    }

    async fn send_group_msg_to_user(
        &self,
        request: Request<ChatMsg>,
    ) -> Result<Response<SendMsgResponse>, Status> {
        let _msg = request.into_inner();

        // self.manager.send_group(&msg.sender_id, &msg).await;

        todo!()
    }
}
