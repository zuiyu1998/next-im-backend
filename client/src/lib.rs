mod error;

use abi::{
    config::{Config, MsgServerConfig},
    message::{tcp::TcpMessageConnector, Message, MessageConnector},
    pb::message::MsgRoute,
    reqwest,
    serde_json::json,
    utils::msg_route_to_url,
    UserId,
};
pub use error::*;

pub struct Client {
    config: MsgServerConfig,
}

impl Client {
    pub fn from_config(config: &Config) -> Self {
        let msg_server_config = config.msg_server.clone();
        Client {
            config: msg_server_config,
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

        let (_stream, _slink) = message.split();

        loop {
            //todo
        }
    }
}
