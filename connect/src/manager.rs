use std::sync::Arc;

use abi::{dashmap::DashMap, message::UserId, pb::message::Msg, tracing::error};

use crate::client::Client;

pub type Hub = Arc<DashMap<UserId, Client>>;

pub struct Manager {
    pub hub: Hub,
}

impl Manager {
    pub async fn send_group(&self, ids: &Vec<UserId>, msg: &Msg) {
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
            if let Err(e) = stream.send(msg).await {
                error!("msg send error: {:?}", e);
            }
        }
    }
}
