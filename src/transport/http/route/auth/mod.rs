pub mod handler;

use std::sync::Arc;

use axum::{Router, routing};

use crate::transport::http::state::ApiState;

const AUTH_ENDPOINT: &str = "/auth";
const REFRESH_COOKIE: &str = "refresh";

pub fn build() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/register", routing::post(handler::register))
        .route("/login", routing::post(handler::login))
        .route("/refresh", routing::post(handler::refresh))
        .route("/me", routing::get(handler::me))
}
