use axum::{extract::State, response::IntoResponse, Json};

use crate::{handlers::json_helper, state::AppState, ErrorKind, Result};

use super::model::UserTokenReq;

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<UserTokenReq>,
) -> Result<impl IntoResponse> {
    let token = state
        .cache
        .get_user_token(req.id)
        .await?
        .ok_or(ErrorKind::UserTokenNotFound)?;

    if token != req.token {
        return Err(ErrorKind::UserTokenInvaild.into());
    }

    Ok(json_helper(()))
}
