use std::sync::Arc;

use abi::{bincode, dashmap::DashMap, pb::message::Msg, tracing::error};

use crate::client::{Client, UserId};

pub type Hub = Arc<DashMap<UserId, Client>>;

pub struct Manager {
    pub hub: Hub,
}

impl Manager {
    pub async fn send_group(&self, ids: &Vec<UserId>, mut msg: Msg) {
        for id in ids {
            if let Some(clients) = self.hub.get(id) {
                self.send_msg_to_clients(&clients, &msg).await;
            }
        }
    }

    pub async fn send_single_msg(&self, id: &UserId, msg: &Msg) {
        if let Some(clients) = self.hub.get(id) {
            self.send_msg_to_clients(&clients, msg).await;
        }
    }

    async fn send_msg_to_clients(&self, clients: &Client, msg: &Msg) {
        for (_, stream) in clients.streams.iter() {
            let content = match bincode::serialize(msg) {
                Ok(res) => res,
                Err(_) => {
                    error!("msg serialize error");
                    return;
                }
            };
            if let Err(e) = client.value().send_binary(content).await {
                error!("msg send error: {:?}", e);
            }
        }
    }
}
