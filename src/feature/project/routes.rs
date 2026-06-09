use std::sync::Arc;

use axum::{Router, routing};

use crate::{feature::project::handler, shared::ApiState};

pub fn routes() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/", routing::post(handler::create))
        .route("/", routing::get(handler::get_by_account))
        .route("/{id}", routing::get(handler::get))
        .route("/{id}", routing::patch(handler::update))
        .route(
            "/{id}/transcript",
            routing::post(handler::create_transcript),
        )
        .route("/{id}/transcript", routing::get(handler::get_transcript))
        .route("/{id}/chat", routing::post(handler::chat))
        .route(
            "/{id}/chat/history",
            routing::get(handler::get_chat_history),
        )
}
