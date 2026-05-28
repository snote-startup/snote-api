use std::sync::Arc;

use axum::{Router, routing};

use crate::transport::http::state::ApiState;

pub fn build() -> Router<Arc<ApiState>> {
    Router::new().route("/health", routing::get(health))
}

async fn health() -> &'static str {
    "ok"
}
