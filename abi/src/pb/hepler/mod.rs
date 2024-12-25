mod chat_msg_builder;

use super::message::{
    login_response::LoginResponseState, msg::Union, LoginRequest, LoginResponse, Msg, Ping,
    Platfrom, Pong,
};

pub use chat_msg_builder::ChatMsgBuilder;

pub fn ping() -> Msg {
    Msg {
        union: Some(Union::Ping(Ping {})),
    }
}

pub fn pong() -> Msg {
    Msg {
        union: Some(Union::Pong(Pong {})),
    }
}

pub fn login(user_id: i64, token: &str, platfrom: Platfrom) -> Msg {
    Msg {
        union: Some(Union::LoginReq(LoginRequest {
            user_id,
            token: token.to_owned(),
            platfrom: platfrom.into(),
        })),
    }
}

pub fn login_res(state: LoginResponseState, error: Option<String>) -> Msg {
    Msg {
        union: Some(Union::LoginRes(LoginResponse {
            state: state.into(),
            error,
        })),
    }
}