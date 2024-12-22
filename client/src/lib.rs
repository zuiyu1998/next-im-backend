mod error;

use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

use abi::{
    config::{ApiConfig, Config},
    message::{tcp::TcpMessageConnector, Message, MessageConnector, MessageSink, MessageStream},
    pb::{
        hepler::{login, ping, pong},
        message::{msg::Union, Msg, MsgRoute, Platfrom},
    },
    reqwest,
    serde_json::{self, json, Value},
    tokio::{
        self,
        sync::{Mutex, RwLock},
    },
    tracing,
    utils::msg_route_to_url,
    UserId,
};
pub use error::*;

static CLIENT: OnceLock<Arc<Mutex<Client>>> = OnceLock::new();

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
    config: ApiConfig,
    shard_sink: Option<Arc<RwLock<Box<dyn MessageSink>>>>,
}

impl Client {
    pub fn from_config(config: &Config) -> Self {
        let api_config = config.api.clone();
        Client {
            config: api_config,
            shard_sink: None,
        }
    }

    pub async fn api_login(&self, id: UserId, token: &str) -> Result<Box<dyn Message>> {
        let client = reqwest::Client::new();
        let json_value = json!({
            "id": id,
            "token": token
        });

        let url = format!("{}/user/login", self.config.http());

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

    pub async fn login(
        &mut self,
        id: UserId,
        token: &str,
        stream: &mut Box<dyn MessageStream>,
        sink: &mut Box<dyn MessageSink>,
    ) -> Result<()> {
        sink.send_msg(&login(id, token, Platfrom::Windows)).await?;

        if let Some(Msg {
            union: Some(Union::LoginRes(res)),
        }) = stream.next_ms(1500).await?
        {
            if res.error.is_some() {
                return Err(ErrorKind::ServerError(res.error.unwrap()).into());
            } else {
                Ok(())
            }
        } else {
            Err(ErrorKind::MsgInvaild.into())
        }
    }

    pub async fn connect(&mut self, id: UserId, token: &str) -> Result<()> {
        let message = self.api_login(id, token).await?;
        let (mut stream, mut sink) = message.split();

        self.login(id, token, &mut stream, &mut sink).await?;

        let shard_sink = Arc::new(RwLock::new(sink));

        self.shard_sink = Some(shard_sink.clone());

        let cloned_tx = shard_sink.clone();
        let mut ping_task = tokio::spawn(async move {
            loop {
                if let Err(e) = cloned_tx.write().await.send_msg(&ping()).await {
                    tracing::error!("send ping error：{:?}", e);
                    // break this task, it will end this conn
                    break;
                }
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        });

        let cloned_tx = shard_sink.clone();

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
                        //todo
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

        Ok(())
    }
}
