pub mod message;
pub mod hepler {
    use super::message::{msg::Union, Msg, Ping, Pong};

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
}
