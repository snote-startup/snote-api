use std::sync::Arc;

use axum::{Router, routing};

use crate::{feature::chat::handler, shared::ApiState};

pub fn routes() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/project/{id}/chat", routing::post(handler::chat))
        .route(
            "/project/{id}/chat/history",
            routing::get(handler::get_history),
        )
}
