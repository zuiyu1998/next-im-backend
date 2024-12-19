mod common;
mod platform;

pub mod tcp;

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use url::Url;

use crate::{
    pb::message::{msg::Union, ChatMsg, Msg, MsgRouteType},
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

pub struct MessageWrapper<R, W> {
    reader: Arc<Mutex<Option<R>>>,
    writer: Arc<Mutex<Option<W>>>,
    info: MessageInfo,
}

impl<R, W> MessageWrapper<R, W> {
    pub fn new(reader: R, writer: W, info: MessageInfo) -> Self {
        MessageWrapper {
            reader: Arc::new(Mutex::new(Some(reader))),
            writer: Arc::new(Mutex::new(Some(writer))),
            info,
        }
    }
}

impl<R, W> Message for MessageWrapper<R, W>
where
    R: MessageStream + Send + 'static,
    W: MessageSink + Send + 'static,
{
    fn split(&self) -> (Box<dyn MessageStream>, Box<dyn MessageSink>) {
        let reader = self.reader.lock().unwrap().take().unwrap();
        let writer = self.writer.lock().unwrap().take().unwrap();
        (Box::new(reader), Box::new(writer))
    }

    fn info(&self) -> MessageInfo {
        self.info.clone()
    }
}

#[derive(Debug, Clone)]
pub struct MessageInfo {
    pub msg_route_type: MsgRouteType,
    pub local_addr: Option<Url>,
    pub remote_addr: Option<Url>,
}

pub trait Message: Send {
    fn split(&self) -> (Box<dyn MessageStream>, Box<dyn MessageSink>);

    fn info(&self) -> MessageInfo;
}

#[async_trait]
pub trait MessageListener: Send {
    async fn listen(&mut self) -> Result<()>;
    async fn accept(&mut self) -> Result<Box<dyn Message>>;
    fn local_url(&self) -> Url;
}

//客户端
#[async_trait]
pub trait MessageConnector: Send {
    async fn connect(&mut self) -> Result<Box<dyn Message>>;
    fn remote_url(&self) -> Url;
    fn set_bind_addrs(&mut self, _addrs: Vec<SocketAddr>) {}
    fn set_ip_version(&mut self, _ip_version: IpVersion) {}
}

#[async_trait]
pub trait MessageSink: 'static + Send + Sync {
    async fn send_msg(&mut self, msg: &Msg) -> Result<()>;

    async fn send_chat_msg(&mut self, msg: &ChatMsg) -> Result<()> {
        let msg = Msg {
            union: Some(Union::ChatMsg(msg.clone())),
        };
        self.send_msg(&msg).await?;
        Ok(())
    }
}

#[async_trait]
pub trait MessageStream: 'static + Send + Sync {
    async fn next_msg(&mut self) -> Result<Option<Msg>>;

    async fn next_ms(&mut self, ms: u64) -> Result<Option<Msg>> {
        let duration = std::time::Duration::from_millis(ms);

        let res = timeout(duration, self.next_msg())
            .await
            .map_err(|_| ErrorKind::Timeout)?;

        res
    }
}
