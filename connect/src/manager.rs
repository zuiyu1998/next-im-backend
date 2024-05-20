use std::sync::Arc;

use abi::{
    dashmap::DashMap,
    pb::message::{msg::Union, ChatMsg, Msg},
    tokio::{
        self,
        sync::mpsc::{self, Receiver, Sender},
    },
    tracing, UserId,
};

use cache::{get_cache, Cache};

use crate::client::Client;

pub type Hub = Arc<DashMap<UserId, Client>>;

pub type MsgSender = Sender<Msg>;
pub type MsgReceiver = Receiver<Msg>;

#[derive(Clone)]
pub struct Manager {
    pub hub: Hub,
    pub cache: Arc<dyn Cache>,
    pub msg_sender: MsgSender,
}

impl Manager {
    pub async fn new() -> Self {
        let (sender, receiver) = mpsc::channel(1024);

        let cache = get_cache();

        let manager = Manager {
            msg_sender: sender,
            cache,
            hub: Default::default(),
        };

        let mut manager_clone = manager.clone();

        tokio::spawn(async move {
            manager_clone.run(receiver).await;
        });

        manager
    }

    pub async fn run(&mut self, mut receiver: MsgReceiver) {
        loop {
            if let Ok(msg) = receiver.try_recv() {
                let Msg { union } = msg;

                if let Some(union) = union {
                    match union {
                        Union::ChatMsg(chat_msg) => {
                            tracing::debug!("chat_msg: {:?}", chat_msg);
                        }
                        _ => {}
                    }
                }
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
}
