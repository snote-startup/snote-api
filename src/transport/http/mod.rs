mod doc;
mod error;
mod extractor;
mod route;
mod state;

use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use tokio::net::TcpListener;

use crate::{config::CONFIG, transport::http::state::ApiState};

fn build(state: Arc<ApiState>) -> Router {
    Router::new()
        .nest("/health", route::health::build())
        .nest("/auth", route::auth::build())
        .nest("/project", route::project::build())
        .merge(doc::build())
        .with_state(state)
}

pub async fn run() -> color_eyre::Result<()> {
    let state = ApiState::new().await?;
    let app = build(state);

    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), CONFIG.port)).await?;

    tracing::info!("Listening on port {}", CONFIG.port);

    axum::serve(listener, app).await?;

    Ok(())
}
