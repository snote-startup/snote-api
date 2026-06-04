use std::sync::Arc;

use axum::routing::Router;
use tower_http::trace::TraceLayer;

use crate::transport::http::state::ApiState;

pub fn trace() -> Router<Arc<ApiState>> {
    Router::new().layer(TraceLayer::new_for_http())
}
