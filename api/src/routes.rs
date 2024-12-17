use crate::state::AppState;
use axum::{routing::post, Router};
use tower_http::trace::TraceLayer;

use crate::handlers::admin;
use crate::handlers::users;

type RouterInstace = Router<AppState>;

pub fn admin_routes() -> RouterInstace {
    Router::new().route("/set_user_token", post(admin::set_user_token))
}

pub fn user_routes() -> RouterInstace {
    Router::new().route("/login", post(users::login))
}

pub fn app_routes() -> RouterInstace {
    Router::new()
        .nest("/user", user_routes())
        .nest("/admin", admin_routes())
        .layer(TraceLayer::new_for_http())
}
