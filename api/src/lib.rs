mod handlers;
mod routes;

pub mod error;

use std::net::SocketAddr;

use abi::{tokio, tracing};
use error::*;

use crate::routes::app_routes;

pub async fn start() -> Result<()> {
    let listener = tokio::net::TcpListener::bind(&"127.0.0.1:6143")
        .await
        .unwrap();

    let app = app_routes();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
