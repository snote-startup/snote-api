use std::{net::SocketAddr, sync::Arc};

use axum::{Router, routing};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{
    doc,
    feature::{auth, chat, project, task},
    shared::{ApiState, Config, health, middleware},
};

pub async fn run() -> color_eyre::Result<()> {
    let config = Config::new()?;
    let port = config.port;
    let origins = config.origins.clone();

    let state = Arc::new(ApiState::new(config).await?);
    let app = Router::new()
        .route("/health", routing::get(health::health))
        .merge(auth::routes())
        .merge(project::routes())
        .merge(chat::routes())
        .merge(task::routes())
        .layer(middleware::cors(&origins))
        .layer(TraceLayer::new_for_http())
        .merge(doc::build())
        .with_state(state);

    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), port)).await?;

    tracing::info!("Listening on port {}", port);

    axum::serve(listener, app).await?;

    Ok(())
}
