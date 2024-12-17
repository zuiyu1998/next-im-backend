use abi::UserId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserTokenReq {
    pub id: UserId,
    pub token: Option<String>,
}
