use std::sync::Arc;

use axum::{Router, routing};

use crate::{feature::auth::handler, shared::ApiState};

pub fn routes() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/auth/register", routing::post(handler::register))
        .route("/auth/login", routing::post(handler::login))
        .route("/auth/refresh", routing::post(handler::refresh))
        .route("/auth/me", routing::get(handler::me))
}
