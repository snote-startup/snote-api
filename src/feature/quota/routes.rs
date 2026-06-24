use std::sync::Arc;

use axum::{Router, routing};

use crate::{feature::quota::handler, shared::ApiState};

pub fn routes() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/quota/buy", routing::post(handler::buy))
        .route("/quota", routing::get(handler::get))
        .route(
            "/quota/payment/return",
            routing::get(handler::handle_payment_return),
        )
}
