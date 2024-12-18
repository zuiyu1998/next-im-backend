use super::ApiMsgService;
use abi::{
    pb::message::{LoginRequest, LogoutRequest},
    tonic::async_trait,
    UserId,
};

pub struct HttpApiMsgService {
    pub host: String,
    pub port: u16,
}

#[async_trait]
impl ApiMsgService for HttpApiMsgService {
    async fn login(&self, login: LoginRequest) -> Option<UserId> {
        if login.username == "lw" {
            return Some(1);
        } else {
            return Some(2);
        }
    }

    async fn logout(&self, _logout: LogoutRequest) {}
}
