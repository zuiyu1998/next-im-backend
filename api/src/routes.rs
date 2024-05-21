use axum::{routing::post, Router};

use crate::handlers::users::login;

pub fn user_routes() -> Router {
    Router::new().route("/login", post(login))
}

pub fn app_routes() -> Router {
    Router::new().nest("/user", user_routes())
}
