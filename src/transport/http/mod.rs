mod error;
mod state;

use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;

use crate::{config::CONFIG, transport::http::state::ApiState};

async fn build() -> color_eyre::Result<Router> {
    let state = ApiState::new().await?;

    Ok(Router::new().with_state(state))
}

pub async fn run() -> color_eyre::Result<()> {
    let api = build().await?;

    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), CONFIG.port)).await?;

    tracing::info!("Listening on port {}", CONFIG.port);

    axum::serve(listener, api).await?;

    Ok(())
}
