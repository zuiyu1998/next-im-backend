mod handlers;
mod routes;
mod state;

pub mod error;

use std::{net::SocketAddr, sync::Arc};

use abi::{config::Config, tokio, tracing};
use cache::get_cache;
use error::*;
use state::AppState;

use crate::routes::app_routes;

pub async fn start(config: &Config) -> Result<()> {
    let cache = get_cache(config);
    let app_state = AppState {
        cache,
        config: Arc::new(config.clone()),
    };

    let app = app_routes().with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&config.api.addr())
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
