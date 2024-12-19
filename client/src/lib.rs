mod error;

use std::{
    sync::{Arc, OnceLock, Mutex},
    time::Duration,
};

use abi::{
    config::{ApiConfig, Config},
    message::{tcp::TcpMessageConnector, Message, MessageConnector, MessageSink},
    pb::{
        hepler::{ping, pong},
        message::{msg::Union, MsgRoute},
    },
    reqwest,
    serde_json::json,
    tokio::{self, sync::RwLock},
    tracing,
    utils::msg_route_to_url,
    UserId,
};
pub use error::*;

static CLIENT: OnceLock<Mutex<Client>> = OnceLock::new();

pub struct IMClient;

impl IMClient {
    pub fn from_config(config: &Config) {
        let client = Client::from_config(config);
        let _ = CLIENT.set(Mutex::new(client));
    }
  
    pub async fn connect(&mut self, id: UserId, token: &str) -> Result<()> {
       let mut guard = CLIENT.get().unwrap().lock().unwrap();
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

    pub async fn login(&self, id: UserId, token: &str) -> Result<Box<dyn Message>> {
        let client = reqwest::Client::new();
        let json_value = json!({
            "id": id,
            "token": token
        });

        let route: MsgRoute = client
            .post(self.config.http())
            .json(&json_value)
            .send()
            .await?
            .json()
            .await?;

        let mut connector = TcpMessageConnector::new(msg_route_to_url(route));

        let message = connector.connect().await?;
        Ok(message)
    }

    pub async fn connect(&mut self, id: UserId, token: &str) -> Result<()> {
        let message = self.login(id, token).await?;
        let (mut stream, sink) = message.split();

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
