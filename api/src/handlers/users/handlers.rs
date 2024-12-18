use abi::pb::message::{MsgRoute, MsgRouteType};
use axum::{extract::State, response::IntoResponse, Json};

use crate::{handlers::json_helper, state::AppState, ErrorKind, Result};

use super::model::UserLoginReq;

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<UserLoginReq>,
) -> Result<impl IntoResponse> {
    let token = state
        .cache
        .get_user_token(req.id)
        .await?
        .ok_or(ErrorKind::UserTokenNotFound)?;

    if token != req.token {
        return Err(ErrorKind::UserTokenInvaild.into());
    }

    let addr = state.config.msg_server.addr();

    let route = MsgRoute {
        route_type: MsgRouteType::Tcp as i32,
        addr,
    };

    Ok(json_helper(route))
}
