use std::sync::Arc;

use chrono::{DateTime, Local};
use nacos_rust_client::client::{
    naming_client::{Instance, ServiceInstanceKey},
    ClientBuilder, NamingClient,
};
use nacos_tonic_discover::TonicDiscoverFactory;
use url::Url;

use crate::{
    config::{Config, ServiceType},
    pb::message::{
        chat_producer_service_client::ChatProducerServiceClient, MsgRoute, MsgRouteType,
    },
    tonic::transport::Channel,
    Error, Result,
};

pub fn get_now() -> DateTime<Local> {
    Local::now()
}

pub type ChatProducerGrpcClient = ChatProducerServiceClient<Channel>;

pub fn msg_route_to_url(route: MsgRoute) -> Url {
    let route_type: MsgRouteType = MsgRouteType::try_from(route.route_type).unwrap();

    match route_type {
        MsgRouteType::Tcp => Url::parse(&format!("tcp://{}", route.addr)).unwrap(),
    }
}

pub trait GrpcClient {
    fn get_service_type() -> ServiceType;

    fn new_client(channel: Channel) -> Self;
}

impl GrpcClient for ChatProducerServiceClient<Channel> {
    fn get_service_type() -> ServiceType {
        ServiceType::Chat
    }

    fn new_client(channel: Channel) -> Self {
        ChatProducerGrpcClient::new(channel)
    }
}

///向Nacos 注册服务
pub fn register_service(config: &Config, service_type: ServiceType) {
    let naming_client = ClientBuilder::new()
        .set_endpoint_addrs(&config.service_center.endpoint_addrs())
        .set_tenant(config.service_center.teant.to_owned())
        .set_use_grpc(true) //select communication protocol
        .build_naming_client();

    let rpc_config = service_type.get_rpc_config(config);

    let instance = Instance::new_simple(
        &rpc_config.ip,
        rpc_config.port,
        &rpc_config.service_name,
        &rpc_config.group_name,
    );

    naming_client.register(instance);
}

pub async fn get_rpc_client<T: GrpcClient>(config: &Config) -> Result<T> {
    let naming_client = init_service_center(config);
    let _ = TonicDiscoverFactory::new(naming_client.clone());

    let rpc_config = T::get_service_type().get_rpc_config(config);

    let service_key = ServiceInstanceKey::new(&rpc_config.service_name, &rpc_config.group_name);

    let discover_factory = nacos_tonic_discover::get_last_factory().unwrap();

    let channel = discover_factory
        .build_service_channel(service_key.clone())
        .await
        .map_err(|e| Error::NacosError(e.to_string()))?;

    Ok(T::new_client(channel))
}

pub fn init_service_center(config: &Config) -> Arc<NamingClient> {
    let naming_client = NamingClient::new_with_addrs(
        &config.service_center.endpoint_addrs(),
        config.service_center.teant.to_owned(),
        None,
    );

    naming_client
}
