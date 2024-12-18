use abi::UserId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserLoginReq {
    pub id: UserId,
    pub token: String,
}
