mod error;

use std::{sync::Arc, time::Duration};

use abi::{
    config::{Config, MsgServerConfig},
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

pub const HEART_BEAT_INTERVAL: u64 = 30;

#[derive(Clone)]
pub struct Client {
    config: MsgServerConfig,
    shard_sink: Option<Arc<RwLock<Box<dyn MessageSink>>>>,
}

impl Client {
    pub fn from_config(config: &Config) -> Self {
        let msg_server_config = config.msg_server.clone();
        Client {
            config: msg_server_config,
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
                tokio::time::sleep(Duration::from_secs(HEART_BEAT_INTERVAL)).await;
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
