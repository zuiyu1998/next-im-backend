use crate::service_discovery::LbWithServiceDiscovery;

use abi::pb::message::{
    chat_service_client::ChatServiceClient, msg_service_client::MsgServiceClient,
};

pub trait ClientFactory {
    // 获取对应的grpc客户端
    fn new_client(channel: LbWithServiceDiscovery) -> Self;
}

impl ClientFactory for ChatServiceClient<LbWithServiceDiscovery> {
    fn new_client(channel: LbWithServiceDiscovery) -> Self {
        Self::new(channel)
    }
}

impl ClientFactory for MsgServiceClient<LbWithServiceDiscovery> {
    fn new_client(channel: LbWithServiceDiscovery) -> Self {
        Self::new(channel)
    }
}
