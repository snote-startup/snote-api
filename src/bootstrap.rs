use std::{future::ready, net::SocketAddr, sync::Arc, time::Duration};

use axum::{Router, routing};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{
    doc,
    feature::{auth, chat, payment_test, project, quota, task},
    shared::{ApiState, Config, health, middleware},
};

const EXPONENTIAL_SECONDS: &[f64] = &[
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
];

pub async fn run_metrics(port: u16) -> color_eyre::Result<()> {
    let recorder_handle = PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap();

    let upkeep_handle = recorder_handle.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            upkeep_handle.run_upkeep();
        }
    });
    let app = Router::new().route(
        "/metrics",
        axum::routing::get(move || ready(recorder_handle.render())),
    );

    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), port)).await?;

    tracing::info!("Metrics listening on port {}", port);

    axum::serve(listener, app).await?;

    todo!()
}

async fn run_main(config: Config) -> color_eyre::Result<()> {
    let port = config.port;
    let origins = config.origins.clone();

    let state = Arc::new(ApiState::new(config).await?);
    let app = Router::new()
        .route("/health", routing::get(health::health))
        .merge(auth::routes())
        .merge(project::routes())
        .merge(chat::routes())
        .merge(task::routes())
        .merge(quota::routes())
        .merge(payment_test::routes())
        .layer(middleware::cors(&origins))
        .layer(TraceLayer::new_for_http())
        .route_layer(axum::middleware::from_fn(middleware::track_metrics))
        .merge(doc::build())
        .with_state(state);

    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), port)).await?;

    tracing::info!("Listening on port {}", port);

    axum::serve(listener, app).await?;

    Ok(())
}

pub async fn run() -> color_eyre::Result<()> {
    let config = Config::new()?;

    tokio::try_join! {
       run_metrics(config.metrics_port),
       run_main(config)
    }?;

    Ok(())
}
