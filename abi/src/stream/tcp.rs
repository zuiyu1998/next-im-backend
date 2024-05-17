use std::sync::Arc;

use tokio::sync::Mutex;
use tokio_util::codec::Framed;

use crate::{
    bincode,
    bytes::Bytes,
    codec::LengthCodec,
    futures::{SinkExt, StreamExt},
    pb::message::{Msg, Platfrom},
    tokio::net::TcpStream as TokioTcpStream,
    tonic::async_trait,
    Result,
};

use super::MessageStream;

pub type TcpConnect = Arc<Mutex<Framed<TokioTcpStream, LengthCodec>>>;

pub struct TcpStream {
    platform: Platfrom,
    connect: TcpConnect,
}

impl TcpStream {
    pub fn new_platform(stream: TokioTcpStream, platform: Platfrom) -> Self {
        let connect = Arc::new(Mutex::new(Framed::new(stream, LengthCodec::default())));

        TcpStream { connect, platform }
    }

    pub fn new(stream: TokioTcpStream) -> Self {
        let connect = Arc::new(Mutex::new(Framed::new(stream, LengthCodec::default())));

        TcpStream {
            connect,
            platform: Platfrom::Unknow,
        }
    }
}

#[async_trait]
impl MessageStream for TcpStream {
    fn get_platfrom(&self) -> Platfrom {
        self.platform
    }

    async fn next(&self) -> Result<Option<Msg>> {
        let mut guard = self.connect.lock().await;

        if let Some(data) = guard.next().await {
            let data = data?;

            let msg = bincode::deserialize::<Msg>(&data)?;

            Ok(Some(msg))
        } else {
            Ok(None)
        }
    }

    async fn send(&self, msg: &Msg) -> Result<()> {
        let data = bincode::serialize(msg)?;

        let bytes = Bytes::copy_from_slice(&data);

        let mut guard = self.connect.lock().await;

        guard.send(bytes).await?;

        Ok(())
    }
}
