use std::sync::Arc;

use axum::{Router, routing};

use crate::{feature::project::handler, shared::ApiState};

pub fn routes() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/project", routing::post(handler::create))
        .route("/project", routing::get(handler::get_by_account))
        .route("/project/{id}", routing::get(handler::get))
        .route("/project/{id}", routing::patch(handler::update))
        .route("/project/{id}/upload", routing::post(handler::upload_audio))
        .route("/project/{id}/stream", routing::any(handler::stream_audio))
        .route(
            "/project/{id}/transcript",
            routing::post(handler::create_transcript),
        )
        .route(
            "/project/{id}/transcript",
            routing::get(handler::get_transcript),
        )
}
