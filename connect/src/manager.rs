use std::{collections::HashMap, sync::Arc};

use abi::{
    dashmap::DashMap,
    pb::message::ChatMsg,
    stream::MessageStream,
    tokio::{
        self,
        sync::mpsc::{self, Receiver, Sender},
    },
    tracing, UserId,
};

use cache::{get_cache, Cache};

use crate::client::Client;

pub type Hub = Arc<DashMap<UserId, Client>>;

pub type ChatMsgSender = Sender<ChatMsg>;
pub type ChatMsgReceiver = Receiver<ChatMsg>;

#[derive(Clone)]
pub struct Manager {
    pub hub: Hub,
    pub cache: Arc<dyn Cache>,
    pub chat_msg_sender: ChatMsgSender,
}

impl Manager {
    pub fn add_stream(&self, user_id: UserId, stream: Arc<dyn MessageStream>) {
        if let Some(mut client) = self.hub.get_mut(&&user_id) {
            client.streams.insert(stream.get_platfrom(), stream);
        } else {
            let mut streams = HashMap::default();

            streams.insert(stream.get_platfrom(), stream);

            self.hub.insert(user_id, Client { user_id, streams });
        }
    }

    pub async fn new() -> Self {
        let (sender, receiver) = mpsc::channel(1024);

        let cache = get_cache();

        let manager = Manager {
            chat_msg_sender: sender,
            cache,
            hub: Default::default(),
        };

        let mut manager_clone = manager.clone();

        tokio::spawn(async move {
            manager_clone.run(receiver).await;
        });

        manager
    }

    pub async fn run(&mut self, mut receiver: ChatMsgReceiver) {
        loop {
            if let Ok(msg) = receiver.try_recv() {
                tracing::debug!("chat_msg: {:?}", msg);
            }
        }
    }

    pub async fn send_group(&self, ids: &Vec<UserId>, msg: &ChatMsg) {
        for id in ids {
            if let Some(clients) = self.hub.get(id) {
                self.send_msg_to_clients(&clients, &msg).await;
            }
        }
    }

    pub async fn send_single_msg(&self, id: &UserId, msg: &ChatMsg) {
        if let Some(clients) = self.hub.get(id) {
            self.send_msg_to_clients(&clients, msg).await;
        }
    }

    async fn send_msg_to_clients(&self, clients: &Client, msg: &ChatMsg) {
        for (_, stream) in clients.streams.iter() {
            if let Err(e) = stream.send_chat_msg(msg).await {
                tracing::error!("msg send error: {:?}", e);
            }
        }
    }

    pub async fn broadcast(&self, msg: ChatMsg) {
        if let Err(e) = self.chat_msg_sender.send(msg).await {
            tracing::error!("manager broadcast error: {}", e);
        }
    }
}
