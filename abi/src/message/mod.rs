mod common;
mod platform;

pub mod tcp;

use std::{net::SocketAddr, pin::Pin};
use url::Url;

use crate::{
    pb::message::{msg::Union, ChatMsg, Msg, Platfrom},
    tokio::time::timeout,
    tonic::async_trait,
    ErrorKind, Result,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IpVersion {
    V4,
    V6,
    Both,
}

pub struct MessageInfo {
    platfrom: Platfrom,
}

pub trait Message: Send {
    fn split(&self) -> (Pin<Box<dyn MessageStream>>, Pin<Box<dyn MessageSlink>>);

    fn get_message_info(&self) -> MessageInfo;
}

#[async_trait]
pub trait MessageListener: Send {
    async fn listen(&mut self) -> Result<()>;
    async fn accept(&mut self) -> Result<Box<dyn Message>>;
    fn local_url(&self) -> Url;
}

//客户端
#[async_trait]
pub trait TunnelConnector: Send {
    async fn connect(&mut self) -> Result<Box<dyn Message>>;
    fn remote_url(&self) -> Url;
    fn set_bind_addrs(&mut self, _addrs: Vec<SocketAddr>) {}
}

#[async_trait]
pub trait MessageSlink: 'static + Send + Sync {
    async fn send(&self, msg: &Msg) -> Result<()>;

    async fn send_chat_msg(&self, msg: &ChatMsg) -> Result<()> {
        let msg = Msg {
            union: Some(Union::ChatMsg(msg.clone())),
        };
        self.send(&msg).await?;
        Ok(())
    }
}

#[async_trait]
pub trait MessageStream: 'static + Send + Sync {
    async fn next(&self) -> Result<Option<Msg>>;

    async fn next_ms(&self, ms: u64) -> Result<Option<Msg>> {
        let duration = std::time::Duration::from_millis(ms);

        let res = timeout(duration, self.next())
            .await
            .map_err(|_| ErrorKind::Timeout)?;

        res
    }
}
