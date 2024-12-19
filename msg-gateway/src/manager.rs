use std::{collections::HashMap, sync::Arc, time::Duration};

use abi::{
    config::Config,
    dashmap::DashMap,
    message::{Message, MessageSink, MessageStream},
    pb::{
        hepler::{login_res, ping, pong},
        message::{msg::Union, ChatMsg, Msg, Platfrom},
    },
    tokio::{
        self,
        sync::{
            mpsc::{Receiver, Sender},
            RwLock,
        },
    },
    tracing,
    utils::{get_rpc_client, ChatServiceGrpcClient},
    UserId,
};

use cache::{get_cache, Cache};

use crate::{client::Client, ErrorKind, Result};

pub type Hub = Arc<DashMap<UserId, Client>>;

pub type ChatMsgSender = Sender<ChatMsg>;
pub type ChatMsgReceiver = Receiver<ChatMsg>;

#[derive(Clone)]
pub struct Manager {
    pub hub: Hub,
    pub cache: Arc<dyn Cache>,
    pub chat_msg_sender: ChatMsgSender,
    pub chat_rpc: ChatServiceGrpcClient,
}

impl Manager {
    pub async fn start_client(
        &mut self,
        user_id: UserId,
        platform: Platfrom,
        mut stream: Box<dyn MessageStream>,
        sink: Box<dyn MessageSink>,
    ) {
        tracing::debug!("start client: user_id-{}, platform-{}", user_id, platform.as_str_name());

        let shard_sink = Arc::new(RwLock::new(sink));

        self.add_sink(user_id, platform, shard_sink.clone());

        let cloned_tx = shard_sink.clone();
        let mut ping_task = tokio::spawn(async move {
            loop {
                if let Err(e) = cloned_tx.write().await.send_msg(&ping()).await {
                    tracing::error!("send ping error：{:?}", e);
                    break;
                }
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        });

        let cloned_tx = shard_sink.clone();
        let sender = self.chat_msg_sender.clone();

        let mut message_task = tokio::spawn(async move {
            while let Ok(Some(msg)) = stream.next_msg().await {
                // 处理消息
                match msg.union.unwrap() {
                    Union::Ping(_) => {
                        if let Err(e) = cloned_tx.write().await.send_msg(&pong()).await {
                            tracing::error!("reply ping error : {:?}", e);
                            break;
                        }
                    }
                    Union::Pong(_) => {
                        // tracing::debug!("received pong message");
                    }
                    Union::ChatMsg(msg) => {
                        tracing::debug!(
                            "msg sender_id:{}, receiver_id: {}",
                            msg.sender_id,
                            msg.receiver_id
                        );

                        if let Err(e) = sender.send(msg).await {
                            tracing::error!("chat_msg_sender send error: {}", e);
                        }
                    }
                    _ => {
                        //todo
                    }
                }
            }
        });

        tokio::select! {
            _ = (&mut message_task) => {
                ping_task.abort();
            },
            _ = (&mut ping_task) => {
                message_task.abort();
            },
        }
    }

    pub async fn handle_message(&mut self, message: Box<dyn Message>) -> Result<()> {
        let (mut stream, mut sink) = message.split();

        if let Some(Msg {
            union: Some(Union::LoginReq(req)),
        }) = stream.next_ms(1500).await?
        {
            if let Some(token) = self.cache.get_user_token(req.user_id).await? {
                if token != req.token {
                    let e = ErrorKind::UseTokenInvaild;
                    sink.send_msg(&login_res(&e.to_string())).await?;
                    return Err(e.into());
                }

                self.start_client(
                    req.user_id,
                    Platfrom::try_from(req.platfrom).unwrap(),
                    stream,
                    sink,
                )
                .await;

                return Ok(());
            } else {
                let e = ErrorKind::UseNotLogin;
                sink.send_msg(&login_res(&e.to_string())).await?;
                return Err(e.into());
            }
        } else {
            let e = ErrorKind::MsgInvaild;
            sink.send_msg(&login_res(&e.to_string())).await?;

            return Err(e.into());
        };
    }

    pub async fn new(config: &Config, chat_msg_sender: ChatMsgSender) -> Self {
        let cache = get_cache(&config);

        let chat_rpc = get_rpc_client(config).await.expect("chat rpc can't open");

        Manager {
            hub: Default::default(),
            cache,
            chat_msg_sender,
            chat_rpc,
        }
    }

    pub fn add_sink(
        &self,
        user_id: UserId,
        platform: Platfrom,
        sink: Arc<RwLock<Box<dyn MessageSink>>>,
    ) {
        if let Some(mut client) = self.hub.get_mut(&&user_id) {
            client.sinks.insert(platform, sink);
        } else {
            let mut sinks = HashMap::default();

            sinks.insert(platform, sink);

            self.hub.insert(user_id, Client { user_id, sinks });
        }
    }

    pub async fn run(&mut self, mut receiver: ChatMsgReceiver) {
        while let Some(msg) = receiver.recv().await {
            tracing::debug!("chat_msg: {:?}", msg);
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
        for (_, sink) in clients.sinks.iter() {
            let mut sink = sink.write().await;

            if let Err(e) = sink.send_chat_msg(msg).await {
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
