pub mod message;
pub mod hepler {
    use super::message::{msg::Union, LoginRequest, LoginResponse, Msg, Ping, Platfrom, Pong};

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

    pub fn login_res(error: &str) -> Msg {
        Msg {
            union: Some(Union::LoginRes(LoginResponse {
                error: error.to_owned(),
            })),
        }
    }
}
