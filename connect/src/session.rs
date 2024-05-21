use std::sync::Arc;

use abi::{
    pb::message::{login_response::LoginResponseCode, msg::Union, LoginResponse, Msg},
    stream::MessageStream,
    tracing,
};

use crate::{manager::Manager, Kind, Result};

pub struct Session {
    pub state: SessionState,
    pub stream: Arc<dyn MessageStream>,
}

pub enum SessionState {
    UnAuthed,
    Authed,
}

impl Session {
    pub fn new(stream: impl MessageStream) -> Self {
        Self {
            state: SessionState::UnAuthed,
            stream: Arc::new(stream),
        }
    }

    pub async fn run(&self, manager: Manager) -> Result<()> {
        loop {
            match self.state {
                SessionState::UnAuthed => {
                    self.hanlde_login(&manager).await?;
                }
                SessionState::Authed => {}
            }
        }
    }

    pub async fn hanlde_login(&self, manager: &Manager) -> Result<()> {
        let msg = self.stream.next().await?.ok_or_else(|| Kind::Timeout)?;

        let Msg { union } = msg;
        let union = union.unwrap();

        match union {
            Union::Login(_req) => {
                //todo user_id

                let user_id = 0;

                tracing::info!("user_id: {} login", user_id);
                manager.add_stream(user_id, self.stream.clone());

                let login_res = LoginResponse {
                    code: LoginResponseCode::Ok as i32,
                    user_id,
                };

                self.stream
                    .send(&Msg {
                        union: Some(Union::LoginRes(login_res)),
                    })
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
