use axum::{extract::State, Json};

use crate::{state::AppState, Result};

use super::model::UserTokenReq;

pub async fn set_user_token(
    State(state): State<AppState>,
    Json(req): Json<UserTokenReq>,
) -> Result<()> {

    state.cache.set_user_token(req.id, req.token).await?;

    Ok(())
}
