use abi::{
    config::{Config, ServiceType},
    synapse::{
        health::HealthCheck,
        service::{
            client::ServiceClient, Scheme, ServiceInstance, ServiceRegistryClient, ServiceStatus,
        },
    },
    tokio::{self, sync::mpsc::Sender},
    tonic::transport::{Channel, Endpoint},
    tracing,
};

use tower::discover::Change;

use std::{net::SocketAddr, time::Duration};

use crate::{
    client_factory::ClientFactory, service_discovery::LbWithServiceDiscovery, Error, Result,
};

//获取host_name
pub fn get_host_name() -> Result<String> {
    let hostname = hostname::get()?;
    let hostname = hostname.into_string().map_err(|_| {
        Error::InternalServer(String::from(
            "get hostname error: OsString into String Failed",
        ))
    })?;
    Ok(hostname)
}

pub async fn register_service(config: &Config, service_type: ServiceType) -> Result<()> {
    // register service to service register center
    let addr = format!(
        "{}://{}:{}",
        config.service_center.protocol, config.service_center.host, config.service_center.port
    );
    let endpoint = Endpoint::from_shared(addr)
        .map_err(|e| Error::TonicError(e.to_string()))?
        .connect_timeout(Duration::from_secs(config.service_center.timeout));
    let mut client = ServiceRegistryClient::connect(endpoint)
        .await
        .map_err(|e| Error::TonicError(e.to_string()))?;

    let (scheme, name, host, port, tags) = match service_type {
        ServiceType::Chat => {
            let scheme = Scheme::from(config.rpc.chat.protocol.as_str()) as i32;
            let name = config.rpc.chat.name.clone();
            let host = config.rpc.chat.host.clone();
            let port = config.rpc.chat.port as i32;
            let tags = config.rpc.chat.tags.clone();
            (scheme, name, host, port, tags)
        }

        ServiceType::Msg => {
            let scheme = Scheme::from(config.rpc.msg.protocol.as_str()) as i32;
            let name = config.rpc.msg.name.clone();
            let host = config.rpc.msg.host.clone();
            let port = config.rpc.msg.port as i32;
            let tags = config.rpc.msg.tags.clone();
            (scheme, name, host, port, tags)
        }

        ServiceType::All => todo!("ALL"),
    };

    //配置心跳
    let mut health_check = None;
    if config.rpc.health_check {
        health_check = Some(HealthCheck {
            endpoint: "".to_string(),
            interval: 10,
            timeout: 10,
            retries: 10,
            scheme,
            tls_domain: None,
        });
    }

    let service = ServiceInstance {
        id: format!("{}-{}", get_host_name()?, name),
        name,
        address: host,
        port,
        tags,
        version: "".to_string(),
        metadata: Default::default(),
        health_check,
        status: 0,
        scheme,
    };
    client.register_service(service).await.unwrap();

    Ok(())
}

async fn get_channel(config: &Config, name: &str) -> Result<LbWithServiceDiscovery> {
    let (channel, sender) = Channel::balance_channel::<SocketAddr>(1024);
    get_channel_(config, name, sender).await?;
    Ok(LbWithServiceDiscovery(channel))
}

async fn get_channel_(
    config: &Config,
    name: &str,
    sender: Sender<Change<SocketAddr, Endpoint>>,
) -> Result<(), Error> {
    let mut client = ServiceClient::builder()
        .server_host(config.service_center.host.clone())
        .server_port(config.service_center.port)
        .connect_timeout(Duration::from_secs(5))
        .build()
        .await
        .map_err(|_| {
            Error::InternalServer("Connect to service register center failed".to_string())
        })?;

    let name = name.to_string();
    tokio::spawn(async move {
        let mut stream = match client.subscribe(name).await {
            Ok(stream) => stream,
            Err(e) => {
                tracing::error!("subscribe channel error: {:?}", e);
                return;
            }
        };
        while let Some(service) = stream.recv().await {
            tracing::debug!("subscribe channel return: {:?}", service);
            let addr = format!("{}:{}", service.address, service.port);
            let socket_addr: SocketAddr = match addr.parse() {
                Ok(sa) => sa,
                Err(e) => {
                    tracing::error!("parse address error:{:?}", e);
                    continue;
                }
            };
            let scheme = Scheme::from(service.scheme as u8);
            let addr = format!("{}://{}", scheme, addr);
            let change = if service.active == ServiceStatus::Up as i32 {
                let endpoint = match Endpoint::from_shared(addr) {
                    Ok(endpoint) => endpoint,
                    Err(err) => {
                        tracing::error!("parse address error:{:?}", err);
                        continue;
                    }
                };
                Change::Insert(socket_addr, endpoint)
            } else {
                Change::Remove(socket_addr)
            };
            if let Err(err) = sender.send(change).await {
                tracing::error!("send channel error:{:?}", err);
                break;
            };
        }
    });
    Ok(())
}

pub async fn get_rpc_client<T: ClientFactory>(config: &Config, service_name: &str) -> Result<T> {
    let channel = get_channel(config, service_name).await?;
    Ok(T::new_client(channel))
}
