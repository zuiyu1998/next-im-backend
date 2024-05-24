use abi::{
    config::{Config, ServiceType},
    tonic::transport::Endpoint,
};

use synapse::{
    health::HealthCheck,
    service::{Scheme, ServiceInstance, ServiceRegistryClient},
};

use std::time::Duration;

use crate::{Error, Result};

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
        .map_err(|e| Error::TonicError(e))?
        .connect_timeout(Duration::from_secs(config.service_center.timeout));
    let mut client = ServiceRegistryClient::connect(endpoint)
        .await
        .map_err(|e| Error::TonicError(e))?;

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
            let scheme = Scheme::from(config.rpc.chat.protocol.as_str()) as i32;
            let name = config.rpc.chat.name.clone();
            let host = config.rpc.chat.host.clone();
            let port = config.rpc.chat.port as i32;
            let tags = config.rpc.chat.tags.clone();
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

pub fn get_rpc_client() {}
