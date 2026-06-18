use std::sync::Arc;

use axum::{Router, routing};

use crate::shared::ApiState;

use super::handler;

pub fn routes() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/project/{id}/task", routing::post(handler::create))
        .route("/project/{id}/task", routing::get(handler::get_by_project))
        .route("/task/{id}", routing::patch(handler::update))
        .route("/task/{id}", routing::delete(handler::delete))
}
