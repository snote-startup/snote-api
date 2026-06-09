use std::{net::SocketAddr, sync::Arc};

use axum::{Router, routing};
use tokio::net::TcpListener;

use crate::{
    doc,
    feature::{auth, project},
    shared::{ApiState, Config, health},
};

fn build(state: Arc<ApiState>) -> Router {
    Router::new()
        .route("/health", routing::get(health::health))
        .nest("/auth", auth::routes())
        .nest("/project", project::routes())
        // .merge(middleware::cors())
        // .merge(middleware::trace())
        .merge(doc::build())
        .with_state(state)
}

pub async fn run() -> color_eyre::Result<()> {
    let config = Config::new()?;
    let port = config.port;

    let state = Arc::new(ApiState::new(config).await?);
    let app = build(state);

    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), port)).await?;

    tracing::info!("Listening on port {}", port);

    axum::serve(listener, app).await?;

    Ok(())
}
