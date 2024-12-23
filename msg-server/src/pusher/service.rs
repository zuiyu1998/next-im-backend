use std::{fmt::Debug, net::SocketAddr, sync::Arc, time::Duration};

use abi::{
    config::{Config, ServiceType},
    dashmap::DashMap,
    nacos_rust_client::client::{
        naming_client::{Instance, QueryInstanceListParams, ServiceInstanceKey},
        NamingClient,
    },
    pb::message::{msg_service_client::MsgServiceClient, ChatMsg},
    tokio::{self, sync::mpsc},
    tonic::{
        async_trait,
        transport::{Channel, Endpoint},
    },
    tracing,
    utils::init_service_center,
};

use super::Pusher;
use crate::{Error, Result};

pub struct PusherService {
    nacos_client: Arc<NamingClient>,
    config: Config,
    msg_rpc_list: Arc<DashMap<SocketAddr, MsgServiceClient<Channel>>>,
}

impl PusherService {
    pub fn new(config: &Config) -> PusherService {
        let nacos_client = init_service_center(config);

        PusherService {
            nacos_client,
            config: config.clone(),
            msg_rpc_list: Default::default(),
        }
    }

    pub async fn get_grpc_list(&self) -> Result<()> {
        let service_type = ServiceType::Msg;

        let rpc_config = service_type.get_rpc_config(&self.config);
        let service_key = ServiceInstanceKey::new(&rpc_config.service_name, &rpc_config.group_name);

        let params = QueryInstanceListParams::new_by_serivce_key(&service_key);

        let instances = self
            .nacos_client
            .query_instances(params)
            .await
            .map_err(|e| Error::NacosError(e.to_string()))?;

        self.handle_sub_services(instances).await;

        Ok(())
    }

    pub async fn handle_sub_services(&self, instances: Vec<Arc<Instance>>) {
        for instance in instances {
            let addr = format!("{}:{}", instance.ip, instance.port);
            let socket: SocketAddr = match addr.parse() {
                Ok(sa) => sa,
                Err(err) => {
                    tracing::error!("parse socket address error: {:?}", err);
                    continue;
                }
            };
            let addr = format!("http://{}:{}", &instance.ip, &instance.port);
            // connect to msg service
            let endpoint = match Endpoint::from_shared(addr) {
                Ok(ep) => ep.connect_timeout(Duration::from_secs(5)),
                Err(err) => {
                    tracing::error!("connect to msg service error: {:?}", err);
                    continue;
                }
            };
            let ws = match MsgServiceClient::connect(endpoint).await {
                Ok(client) => client,
                Err(err) => {
                    tracing::error!("connect to ws service error: {:?}", err);
                    continue;
                }
            };
            self.msg_rpc_list.insert(socket, ws);
        }
    }
}

impl Debug for PusherService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PusherService").finish()
    }
}

#[async_trait]
impl Pusher for PusherService {
    async fn push_single_msg(&self, request: ChatMsg) -> Result<()> {
        tracing::debug!("push msg request: {:?}", request);

        let msg_rpc = self.msg_rpc_list.clone();
        if msg_rpc.is_empty() {
            self.get_grpc_list().await?;
        }

        let (tx, mut rx) = mpsc::channel(msg_rpc.len());

        // send message to ws with asynchronous way
        for v in msg_rpc.iter() {
            let tx = tx.clone();
            let service_id = *v.key();
            let mut v = v.clone();
            let request = request.clone();
            tokio::spawn(async move {
                if let Err(err) = v.send_msg_to_user(request).await {
                    tx.send((service_id, err)).await.unwrap();
                };
            });
        }

        // close tx
        drop(tx);

        self.get_grpc_list().await?;

        // and need to handle error
        while let Some((service_id, err)) = rx.recv().await {
            msg_rpc.remove(&service_id);
            tracing::error!("push msg to {} failed: {}", service_id, err);
        }
        Ok(())
    }
}
