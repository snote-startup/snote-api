pub mod handler;

use std::sync::Arc;

use axum::{Router, routing};

use crate::transport::http::state::ApiState;

pub fn build() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/", routing::post(handler::create))
        .route("/", routing::get(handler::get_by_account))
        .route("/{id}", routing::get(handler::get))
        .route("/{id}/transcripts", routing::get(handler::get_transcripts))
        .route("/{id}", routing::patch(handler::update))
}
