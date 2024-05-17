use crate::manager::Manager;
use abi::{
    message::Message,
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
        let _msg = request
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
        let msg = request
            .into_inner()
            .msg
            .ok_or_else(|| Status::invalid_argument("message is empty"))?;

        let id = msg
            .get_sender_id()
            .ok_or_else(|| Status::invalid_argument("message is invalid"))?;

        self.manager.send_single_msg(&id, &msg).await;

        let response = Response::new(SendMsgResponse {});
        Ok(response)
    }

    async fn send_group_msg_to_user(
        &self,
        request: Request<SendMsgRequest>,
    ) -> Result<Response<SendMsgResponse>, Status> {
        let msg = request
            .into_inner()
            .msg
            .ok_or_else(|| Status::invalid_argument("message is empty"))?;

        let _id = msg
            .get_sender_id()
            .ok_or_else(|| Status::invalid_argument("message is invalid"))?;

        // self.manager.send_group(&id, &msg).await;

        todo!()
    }
}
