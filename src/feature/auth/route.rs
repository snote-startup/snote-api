use std::sync::Arc;

use axum::{Router, routing};

use crate::{feature::auth::handler, state::ApiState};

pub fn routes() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/register", routing::post(handler::register))
        .route("/login", routing::post(handler::login))
        .route("/refresh", routing::post(handler::refresh))
        .route("/me", routing::get(handler::me))
}
