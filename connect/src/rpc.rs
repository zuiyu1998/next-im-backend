use crate::manager::Manager;
use abi::{
    pb::message::{connect_service_server::ConnectService, SendMsgRequest, SendMsgResponse},
    tonic::{async_trait, Request, Response, Status},
};

pub struct ConnectRpcService {
    manager: Manager,
}

#[async_trait]
impl ConnectService for ConnectRpcService {
    async fn send_message(
        &self,
        request: Request<SendMsgRequest>,
    ) -> Result<Response<SendMsgResponse>, Status> {
        let msg = request
            .into_inner()
            .msg
            .ok_or_else(|| Status::invalid_argument("message is empty"))?;
        // self.manager.broadcast(msg).await?;
        let response = Response::new(SendMsgResponse {});
        Ok(response)
    }

    async fn send_msg_to_user(
        &self,
        request: Request<SendMsgRequest>,
    ) -> Result<Response<SendMsgResponse>, Status> {
        todo!()
    }

    async fn send_group_msg_to_user(
        &self,
        request: Request<SendMsgRequest>,
    ) -> Result<Response<SendMsgResponse>, Status> {
        todo!()
    }
}
