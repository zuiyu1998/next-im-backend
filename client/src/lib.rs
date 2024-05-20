mod error;

pub use error::*;

use std::{net::SocketAddr, sync::Arc};

use abi::{
    pb::message::{msg::Union, LoginRequest, Msg, Platfrom},
    stream::{tcp::TcpStream, MessageStream},
    tokio::{net::TcpSocket, sync::Mutex},
    tracing, UserId,
};

#[derive(Default, Clone)]
pub enum ClientState {
    #[default]
    None,
    UnAuthed,
    Authed,
    NetWorkConnecting,
}

pub struct ClientOptions {
    pub addr: SocketAddr,
}

pub struct ClientData {
    pub user_id: UserId,
}

#[derive(Clone)]
pub struct Client {
    pub state: Arc<Mutex<ClientState>>,
    pub stream: Option<Arc<dyn MessageStream>>,
    pub data: Arc<Mutex<Option<ClientData>>>,
    pub options: Arc<ClientOptions>,
}

impl Client {
    pub async fn get_state(&self) -> ClientState {
        let guard = self.state.lock().await;

        guard.clone()
    }

    pub async fn set_state(&self, state: ClientState) {
        let mut guard = self.state.lock().await;
        *guard = state;
    }

    pub async fn set_data(&self, data: ClientData) {
        let mut guard = self.data.lock().await;

        *guard = Some(data)
    }

    fn set_stream(&mut self, stream: impl MessageStream) {
        self.stream = Some(Arc::new(stream));
    }

    pub fn new(options: ClientOptions) -> Self {
        Self {
            state: Default::default(),
            stream: Default::default(),
            options: Arc::new(options),
            data: Default::default(),
        }
    }

    pub async fn run(mut self) {
        loop {
            let state = self.get_state().await;

            match state {
                ClientState::None => {
                    if let Err(e) = self.bind().await {
                        tracing::error!("clinet bind error: {}", e);
                    }
                }

                ClientState::UnAuthed => {}
                _ => {}
            }
        }
    }

    pub async fn bind(&mut self) -> Result<()> {
        let socket = TcpSocket::new_v4()?;

        let stream = socket.connect(self.options.addr).await?;

        tracing::info!("server running on {}", self.options.addr);

        let stream = TcpStream::new(stream);

        self.set_stream(stream);
        self.set_state(ClientState::UnAuthed).await;

        Ok(())
    }

    pub async fn login(
        &mut self,
        username: &str,
        passworld: &str,
        platform: Platfrom,
    ) -> Result<()> {
        assert_eq!(true, self.stream.is_some());

        let req = LoginRequest {
            username: username.to_owned(),
            password: passworld.to_owned(),
            platfrom: platform as i32,
        };

        let stream = self.stream.clone().unwrap();

        stream
            .send(&Msg {
                union: Some(Union::Login(req)),
            })
            .await?;
        if let Some(msg) = stream.next_ms(1000).await? {
            let Msg { union } = msg;
            let union = union.unwrap();

            match union {
                Union::LoginRes(res) => {
                    self.set_state(ClientState::Authed).await;

                    self.set_data(ClientData {
                        user_id: res.user_id,
                    })
                    .await;
                    Ok(())
                }
                _ => Err(Kind::ServerError.into()),
            }
        } else {
            self.set_state(ClientState::None).await;

            Err(Kind::ServerNotResponding.into())
        }
    }
}
