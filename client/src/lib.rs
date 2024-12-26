mod error;

use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

use abi::{
    config::{ApiConfig, Config},
    futures::TryFutureExt,
    message::{tcp::TcpMessageConnector, Message, MessageConnector, MessageSink, MessageStream},
    pb::{
        hepler::{login, ping, pong},
        message::{
            handshake, msg::Union, ChatMsg, Handshake, LoginRequest, LoginResponse, Msg, MsgRoute,
            Platfrom,
        },
    },
    reqwest,
    serde_json::{self, json, Value},
    tokio::{
        self,
        sync::{Mutex, RwLock},
        task::{JoinHandle, JoinSet},
        time::timeout,
    },
    tracing,
    utils::msg_route_to_url,
    UserId,
};
pub use error::*;

static CLIENT: OnceLock<Arc<Mutex<Client>>> = OnceLock::new();

pub struct GlobalCtx {
    config: ApiConfig,
}

pub type ArcGlobalCtx = Arc<GlobalCtx>;

pub struct IMClient;

impl IMClient {
    pub fn from_config(config: &Config) {
        let client = Arc::new(Mutex::new(Client::from_config(config)));
        let _ = CLIENT.set(client);
    }

    pub async fn connect(&mut self, id: UserId, token: &str) -> Result<()> {
        let mut guard = CLIENT.get().unwrap().lock().await;
        guard.connect(id, token).await?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct Client {
    ctx: ArcGlobalCtx,
    peer: Arc<Mutex<Option<Peer>>>,
}

pub struct Peer {
    ctx: ArcGlobalCtx,
    shard_sink: Arc<Mutex<Box<dyn MessageSink>>>,
    id: UserId,
    token: String,
}

pub struct PeerConn {
    ctx: ArcGlobalCtx,
    id: UserId,
    token: String,
    sink: Arc<Mutex<Box<dyn MessageSink>>>,
    stream: Arc<Mutex<Box<dyn MessageStream>>>,
    tasks: JoinSet<()>,
}

impl PeerConn {
    pub fn new(ctx: ArcGlobalCtx, id: UserId, token: &str, message: Box<dyn Message>) -> Self {
        let (stream, sink) = message.split();

        PeerConn {
            ctx,
            id,
            token: token.to_string(),
            sink: Arc::new(Mutex::new(sink)),
            stream: Arc::new(Mutex::new(stream)),
            tasks: JoinSet::default(),
        }
    }

    pub async fn do_handshake(&self) -> Result<()> {
        self.send_handshake().await?;
        tracing::info!("waiting for handshake request from server");
        self.wait_handshake_loop().await?;
        tracing::info!("handshake response success");
        Ok(())
    }

    async fn wait_handshake_loop(&self) -> Result<LoginResponse> {
        timeout(Duration::from_secs(5), async move {
            loop {
                match self.wait_handshake().await? {
                    Some(rsp) => return Ok(rsp),
                    None => {
                        continue;
                    }
                }
            }
        })
        .map_err(|e| Error::WaitRespError(format!("wait handshake timeout: {:?}", e)))
        .await?
    }

    async fn wait_handshake(&self) -> Result<Option<LoginResponse>> {
        let mut stream = self.stream.lock().await;

        if let Some(Msg {
            union:
                Some(Union::Handshake(Handshake {
                    union: Some(handshake::Union::LoginRes(res)),
                })),
        }) = stream.next_msg().await?
        {
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }

    pub async fn send_handshake(&self) -> Result<()> {
        let mut sink = self.sink.lock().await;
        sink.send_msg(&login(self.id, &self.token, Platfrom::Windows))
            .await?;

        Ok(())
    }
}

impl Client {
    pub fn from_config(config: &Config) -> Self {
        let api_config = config.api.clone();
        Client {
            ctx: ArcGlobalCtx::new(GlobalCtx { config: api_config }),
            peer: Default::default(),
        }
    }

    pub async fn send_msg(&self, msg: &ChatMsg) -> Result<()> {
        Ok(())
    }

    async fn api_login(&self, id: UserId, token: &str) -> Result<Box<dyn Message>> {
        let client = reqwest::Client::new();
        let json_value = json!({
            "id": id,
            "token": token
        });

        let url = format!("{}/user/login", self.ctx.config.http());

        let res: Value = client
            .post(url)
            .json(&json_value)
            .send()
            .await?
            .json()
            .await?;

        let route_data = res.get("data").ok_or(ErrorKind::JsonInvaild)?.clone();
        let route: MsgRoute = serde_json::from_value(route_data)?;

        let mut connector = TcpMessageConnector::new(msg_route_to_url(route));

        let message = connector.connect().await?;
        Ok(message)
    }

    pub async fn connect(&mut self, id: UserId, token: &str) -> Result<()> {
        let message = self.api_login(id, token).await?;

        let peer_conn = PeerConn::new(self.ctx.clone(), id, token, message);
        peer_conn.do_handshake().await?;

        self.add_peer_conn(peer_conn).await?;

        Ok(())
    }

    pub async fn add_peer_conn(&mut self, peer_conn: PeerConn) -> Result<()> {
        Ok(())
    }
}
