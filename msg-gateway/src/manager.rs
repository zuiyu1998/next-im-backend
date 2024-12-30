use std::{collections::HashMap, sync::Arc, time::Duration};

use abi::{
    chrono,
    config::Config,
    dashmap::DashMap,
    futures::TryFutureExt,
    message::{Message, MessageSink, MessageStream},
    nanoid::nanoid,
    pb::{
        hepler::{login_res, ping, pong},
        message::{
            handshake, login_response::LoginResponseState, msg::Union, ChatMsg, Handshake, Msg,
            Platfrom,
        },
        session::Session,
    },
    tokio::{
        self,
        sync::{
            mpsc::{Receiver, Sender},
            Mutex,
        },
        task::JoinSet,
        time::timeout,
    },
    tonic::Request,
    tracing,
    utils::{get_rpc_client, ChatProducerGrpcClient},
    UserId,
};

use cache::{get_cache, Cache};

use crate::{Error, ErrorKind, Result};

pub type Hub = Arc<DashMap<UserId, Client>>;

pub type ChatMsgSender = Sender<ChatMsg>;
pub type ChatMsgReceiver = Receiver<ChatMsg>;

pub struct Client {
    pub user_id: UserId,
    pub peers: HashMap<Platfrom, PeerConn>,
}

#[derive(Clone)]
pub struct Manager {
    pub hub: Hub,
    pub cache: Arc<dyn Cache>,
    pub chat_msg_sender: ChatMsgSender,
    pub chat_rpc: ChatProducerGrpcClient,
}

pub struct PeerConn {
    sink: Arc<Mutex<Box<dyn MessageSink>>>,
    stream: Arc<Mutex<Option<Box<dyn MessageStream>>>>,
    tasks: JoinSet<()>,
    platform: Platfrom,
    cache: Arc<dyn Cache>,
    chat_msg_sender: ChatMsgSender,
    user_id: UserId,
}

impl PeerConn {
    pub fn new(
        cache: Arc<dyn Cache>,
        chat_msg_sender: ChatMsgSender,
        message: Box<dyn Message>,
    ) -> Self {
        let (stream, sink) = message.split();

        PeerConn {
            cache,
            chat_msg_sender,
            sink: Arc::new(Mutex::new(sink)),
            stream: Arc::new(Mutex::new(Some(stream))),
            tasks: JoinSet::default(),
            platform: Platfrom::Unknow,
            user_id: 0,
        }
    }

    pub async fn send_chat_msg(&self, chat_msg: &ChatMsg) -> Result<()> {
        let mut sink = self.sink.lock().await;

        sink.send_chat_msg(chat_msg).await?;
        Ok(())
    }

    pub async fn do_handshake(&mut self) -> Result<()> {
        tracing::info!("waiting for handshake request from client");
        self.wait_handshake_loop().await?;
        tracing::info!("handshake response success");
        Ok(())
    }

    async fn wait_handshake(&mut self) -> Result<Option<()>> {
        let mut stream = self.stream.lock().await;
        let sink = self.sink.clone();

        if let Some(Msg {
            union:
                Some(Union::Handshake(Handshake {
                    union: Some(handshake::Union::LoginReq(req)),
                })),
        }) = stream.as_mut().unwrap().next_msg().await?
        {
            if let Some(token) = self.cache.get_user_token(req.user_id).await? {
                if token != req.token {
                    let mut sink = sink.lock().await;
                    let e = ErrorKind::UseTokenInvaild;
                    sink.send_msg(&login_res(LoginResponseState::Fail, Some(e.to_string())))
                        .await?;
                    return Err(e.into());
                } else {
                    self.user_id = req.user_id;
                    self.platform = Platfrom::try_from(self.platform).unwrap();

                    let mut sink = sink.lock().await;
                    sink.send_msg(&login_res(LoginResponseState::Success, None))
                        .await?;
                    return Ok(Some(()));
                }
            } else {
                let mut sink = sink.lock().await;
                let e = ErrorKind::UseNotLogin;
                sink.send_msg(&login_res(LoginResponseState::Fail, Some(e.to_string())))
                    .await?;
                return Err(e.into());
            }
        } else {
            Ok(None)
        }
    }

    async fn wait_handshake_loop(&mut self) -> Result<()> {
        timeout(Duration::from_secs(5), async move {
            loop {
                match self.wait_handshake().await? {
                    Some(_) => return Ok(()),
                    None => {
                        continue;
                    }
                }
            }
        })
        .map_err(|e| Error::WaitRespError(format!("wait handshake timeout: {:?}", e)))
        .await?
    }

    pub async fn handle_chat_msg(&mut self, mut chat_msg: ChatMsg) -> Result<()> {
        tracing::debug!(
            "received msg sender_id:{}, receiver_id: {}",
            chat_msg.sender_id,
            chat_msg.receiver_id
        );

        chat_msg.server_id = nanoid!();

        chat_msg.server_at = chrono::Local::now()
            .naive_local()
            .and_utc()
            .timestamp_millis();

        let session = Session::from_chat_msg(&chat_msg);

        self.cache.get_seq(&session).await?;

        self.chat_msg_sender
            .send(chat_msg)
            .await
            .map_err(|e| Error::SendError(e.to_string()))?;
        Ok(())
    }

    pub async fn start_ping_pong(&mut self) {
        let cloned_tx = self.sink.clone();

        self.tasks.spawn(async move {
            loop {
                if let Err(e) = cloned_tx.lock().await.send_msg(&ping()).await {
                    tracing::error!("send ping error: {:?}", e);
                    break;
                }
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        });
    }

    pub async fn start_recv_loop(&mut self) {
        let mut stream = self.stream.lock().await.take().unwrap();
        let sink = self.sink.clone();

        self.tasks.spawn(async move {
            while let Ok(Some(msg)) = stream.next_msg().await {
                // 处理消息
                match msg.union.unwrap() {
                    Union::Ping(_) => {
                        if let Err(e) = sink.lock().await.send_msg(&pong()).await {
                            tracing::error!("reply ping error : {:?}", e);
                            break;
                        }
                    }
                    Union::Pong(_) => {
                        tracing::debug!("received pong message");
                    }
                    Union::ChatMsg(msg) => {
                        tracing::debug!("received message: {:#?}", msg);
                    }
                    _ => {
                        //todo
                    }
                }
            }
        });
    }
}

impl Manager {
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

    pub async fn handle_message(&mut self, message: Box<dyn Message>) -> Result<()> {
        let mut peer_conn =
            PeerConn::new(self.cache.clone(), self.chat_msg_sender.clone(), message);

        peer_conn.do_handshake().await?;

        self.add_peer_conn(peer_conn).await;

        Ok(())
    }

    pub async fn add_peer_conn(&self, mut peer_conn: PeerConn) {
        peer_conn.start_ping_pong().await;
        peer_conn.start_recv_loop().await;

        let user_id = peer_conn.user_id;

        if let Some(mut client) = self.hub.get_mut(&user_id) {
            client.peers.insert(peer_conn.platform, peer_conn);
        } else {
            let mut peers = HashMap::default();

            peers.insert(peer_conn.platform, peer_conn);

            self.hub.insert(user_id, Client { user_id, peers });
        }
    }

    pub async fn run(&mut self, mut receiver: ChatMsgReceiver) {
        while let Some(msg) = receiver.recv().await {
            tracing::debug!(
                "send chat rpc chat_msg: sender_id: {}, receiver_id: {}",
                msg.sender_id,
                msg.receiver_id
            );

            if let Err(e) = self.chat_rpc.send_message(Request::new(msg)).await {
                tracing::error!("chat rpc send message error: {}", e);
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
        for (_, peer) in clients.peers.iter() {
            if let Err(e) = peer.send_chat_msg(msg).await {
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
