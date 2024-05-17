use crate::manager::Manager;
use abi::{
    pb::message::{msg_service_server::MsgService, ChatMsg, SendMsgResponse},
    tonic::{async_trait, Request, Response, Status},
};

pub struct ConnectRpcService {
    manager: Manager,
}

#[async_trait]
impl MsgService for ConnectRpcService {
    async fn send_message(
        &self,
        request: Request<ChatMsg>,
    ) -> Result<Response<SendMsgResponse>, Status> {
        let _msg = request.into_inner();

        // self.manager.broadcast(msg).await?;
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
