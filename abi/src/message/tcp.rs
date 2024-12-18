use std::{net::SocketAddr, sync::Arc};

use tokio::{net::{TcpListener, TcpSocket}, sync::Mutex};
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

use super::{
    common::{check_scheme_and_get_socket_addr, setup_sokcet2}, Message, MessageListener
};

#[derive(Debug)]
pub struct TcpMessageListener {
    addr: url::Url,
    listener: Option<TcpListener>,
}

impl TcpMessageListener {
    pub fn new(addr: url::Url) -> Self {
        TcpMessageListener {
            addr,
            listener: None,
        }
    }
}

#[async_trait]
impl MessageListener for TcpMessageListener {
    async fn listen(&mut self) -> Result<()> {
        let addr = check_scheme_and_get_socket_addr::<SocketAddr>(&self.addr, "tcp")?;

        let socket2_socket = socket2::Socket::new(
            socket2::Domain::for_address(addr),
            socket2::Type::STREAM,
            Some(socket2::Protocol::TCP),
        )?;
        setup_sokcet2(&socket2_socket, &addr)?;
        let socket = TcpSocket::from_std_stream(socket2_socket.into());

        if let Err(e) = socket.set_nodelay(true) {
            tracing::warn!(?e, "set_nodelay fail in listen");
        }

        self.addr
            .set_port(Some(socket.local_addr()?.port()))
            .unwrap();

        self.listener = Some(socket.listen(1024)?);
        Ok(())
    }

    async fn accept(&mut self) -> Result<Box<dyn Message>> {
        let listener = self.listener.as_ref().unwrap();
        let (stream, _) = listener.accept().await?;

        if let Err(e) = stream.set_nodelay(true) {
            tracing::warn!(?e, "set_nodelay fail in accept");
        }
        let (r, w) = stream.into_split();

        todo!()
        // Ok(Box::new(TunnelWrapper::new(
        //     FramedReader::new(r, TCP_MTU_BYTES),
        //     FramedWriter::new(w),
        //     Some(info),
        // )))
    }

    fn local_url(&self) -> url::Url {
        self.addr.clone()
    }
}
