mod auth;
mod error;
mod health;
mod state;

use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;

use crate::{config::CONFIG, transport::http::state::ApiState};

pub async fn run() -> color_eyre::Result<()> {
    let state = ApiState::new().await?;
    let app = Router::new()
        .nest("/health", health::build())
        .with_state(state);

    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), CONFIG.port)).await?;

    tracing::info!("Listening on port {}", CONFIG.port);

    axum::serve(listener, app).await?;

    Ok(())
}
