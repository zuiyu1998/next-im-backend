mod http;

use abi::{
    pb::message::{LoginRequest, LogoutRequest},
    tonic::async_trait,
    UserId,
};

pub use http::*;

#[async_trait]
pub trait ApiMsgService: 'static + Send + Sync {
    async fn login(&self, login: LoginRequest) -> Option<UserId>;

    async fn logout(&self, logout: LogoutRequest);
}
