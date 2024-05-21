mod http;

use abi::{pb::message::LoginRequest, tonic::async_trait, UserId};
use std::sync::Arc;

pub use http::*;

#[async_trait]
pub trait ApiMsgService: 'static + Send + Sync {
    async fn login(&self, login: LoginRequest) -> Option<UserId>;
}

#[derive(Clone)]
pub struct ApiMsgServiceInstance(Arc<dyn ApiMsgService>);

#[async_trait]
impl ApiMsgService for ApiMsgServiceInstance {
    async fn login(&self, login: LoginRequest) -> Option<UserId> {
        self.0.login(login).await
    }
}

impl ApiMsgServiceInstance {
    pub fn new(service: impl ApiMsgService) -> Self {
        ApiMsgServiceInstance(Arc::new(service))
    }
}
