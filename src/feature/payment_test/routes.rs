use std::sync::Arc;

use axum::{Router, routing};

use crate::{feature::payment_test::handler, shared::ApiState};

pub fn routes() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/payment-test", routing::post(handler::create_link))
        .route("/payment-test/return", routing::get(handler::handle_return))
}
