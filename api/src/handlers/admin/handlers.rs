use axum::{extract::State, response::IntoResponse, Json};

use crate::{state::AppState, Result, handlers::json_helper};

use super::model::UserTokenReq;

pub async fn set_user_token(
    State(state): State<AppState>,
    Json(req): Json<UserTokenReq>,
) -> Result<impl IntoResponse> {

    state.cache.set_user_token(req.id, req.token).await?;

    Ok(json_helper(()))
}
