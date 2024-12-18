use std::net::SocketAddr;

use futures::stream::FuturesUnordered;
use futures::StreamExt;
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpListener, TcpSocket, TcpStream,
};
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::{
    bincode,
    bytes::Bytes,
    codec::LengthCodec,
    futures::SinkExt,
    pb::message::{Msg, MsgRouteType},
    tonic::async_trait,
    Result,
};

use super::{
    common::{
        build_url_from_socket_addr, check_scheme_and_get_socket_addr,
        check_scheme_and_get_socket_addr_ext, setup_sokcet2, wait_for_connect_futures,
    },
    IpVersion, Message, MessageInfo, MessageListener, MessageSlink, MessageStream, MessageWrapper,
};

fn get_tunnel_with_tcp_stream(stream: TcpStream, remote_url: url::Url) -> Result<Box<dyn Message>> {
    if let Err(e) = stream.set_nodelay(true) {
        tracing::warn!(?e, "set_nodelay fail in get_tunnel_with_tcp_stream");
    }

    let info = MessageInfo {
        msg_route_type: MsgRouteType::Tcp,
        local_addr: Some(
            build_url_from_socket_addr(&stream.local_addr()?.to_string(), "tcp").into(),
        ),
        remote_addr: Some(remote_url.into()),
    };
    let codec = LengthCodec::default();

    let (r, w) = stream.into_split();

    Ok(Box::new(MessageWrapper::new(
        FramedRead::new(r, codec.clone()),
        FramedWrite::new(w, codec),
        info,
    )))
}

#[derive(Debug)]
pub struct TcpMessageConnector {
    addr: url::Url,

    bind_addrs: Vec<SocketAddr>,
    ip_version: IpVersion,
}

impl TcpMessageConnector {
    pub fn new(addr: url::Url) -> Self {
        TcpMessageConnector {
            addr,
            bind_addrs: vec![],
            ip_version: IpVersion::Both,
        }
    }

    async fn connect_with_default_bind(&mut self, addr: SocketAddr) -> Result<Box<dyn Message>> {
        tracing::info!(addr = ?self.addr, "connect tcp start");
        let stream = TcpStream::connect(addr).await?;
        tracing::info!(addr = ?self.addr, "connect tcp succ");
        return get_tunnel_with_tcp_stream(stream, self.addr.clone().into());
    }

    async fn connect_with_custom_bind(&mut self, addr: SocketAddr) -> Result<Box<dyn Message>> {
        let futures = FuturesUnordered::new();

        for bind_addr in self.bind_addrs.iter() {
            tracing::info!(bind_addr = ?bind_addr, ?addr, "bind addr");

            let socket2_socket = socket2::Socket::new(
                socket2::Domain::for_address(addr),
                socket2::Type::STREAM,
                Some(socket2::Protocol::TCP),
            )?;

            if let Err(e) = setup_sokcet2(&socket2_socket, bind_addr) {
                tracing::error!(bind_addr = ?bind_addr, ?addr, "bind addr fail: {:?}", e);
                continue;
            }

            let socket = TcpSocket::from_std_stream(socket2_socket.into());
            futures.push(socket.connect(addr.clone()));
        }

        let ret = wait_for_connect_futures(futures).await;
        return get_tunnel_with_tcp_stream(ret?, self.addr.clone().into());
    }
}

#[async_trait]
impl super::MessageConnector for TcpMessageConnector {
    async fn connect(&mut self) -> Result<Box<dyn Message>> {
        let addr =
            check_scheme_and_get_socket_addr_ext::<SocketAddr>(&self.addr, "tcp", self.ip_version)?;
        if self.bind_addrs.is_empty() || addr.is_ipv6() {
            self.connect_with_default_bind(addr).await
        } else {
            self.connect_with_custom_bind(addr).await
        }
    }

    fn remote_url(&self) -> url::Url {
        self.addr.clone()
    }

    fn set_bind_addrs(&mut self, addrs: Vec<SocketAddr>) {
        self.bind_addrs = addrs;
    }

    fn set_ip_version(&mut self, ip_version: IpVersion) {
        self.ip_version = ip_version;
    }
}

#[async_trait]
impl MessageStream for FramedRead<OwnedReadHalf, LengthCodec> {
    async fn next_msg(&mut self) -> Result<Option<Msg>> {
        if let Some(data) = self.next().await {
            let data = data?;

            let msg = bincode::deserialize::<Msg>(&data)?;

            Ok(Some(msg))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl MessageSlink for FramedWrite<OwnedWriteHalf, LengthCodec> {
    async fn send_msg(&mut self, msg: &Msg) -> Result<()> {
        let data = bincode::serialize(msg)?;
        let bytes = Bytes::copy_from_slice(&data);

        self.send(bytes).await?;

        Ok(())
    }
}

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

        let codec = LengthCodec::default();
        let info = MessageInfo {
            msg_route_type: MsgRouteType::Tcp,
            local_addr: Some(self.local_url().into()),
            remote_addr: Some(
                build_url_from_socket_addr(&stream.peer_addr()?.to_string(), "tcp").into(),
            ),
        };

        let (r, w) = stream.into_split();

        Ok(Box::new(MessageWrapper::new(
            FramedRead::new(r, codec.clone()),
            FramedWrite::new(w, codec),
            info,
        )))
    }

    fn local_url(&self) -> url::Url {
        self.addr.clone()
    }
}
