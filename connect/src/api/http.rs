use super::ApiMsgService;
use abi::{pb::message::LoginRequest, tonic::async_trait, UserId};

pub struct HttpApiMsgService;

#[async_trait]
impl ApiMsgService for HttpApiMsgService {
    async fn login(&self, login: LoginRequest) -> Option<UserId> {
        if login.username == "lw" {
            return Some(1);
        } else {
            return Some(2);
        }
    }
}
