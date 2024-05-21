use abi::pb::message::{login_response::LoginResponseCode, LoginRequest, LoginResponse};
use axum::Json;

use crate::Result;

pub async fn login(Json(req): Json<LoginRequest>) -> Result<Json<LoginResponse>> {
    if req.username == "lw" {
        return Ok(Json(LoginResponse {
            code: LoginResponseCode::Ok as i32,
            user_id: 1,
        }));
    } else {
        return Ok(Json(LoginResponse {
            code: LoginResponseCode::Ok as i32,
            user_id: 2,
        }));
    }
}
