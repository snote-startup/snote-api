use std::sync::Arc;

use axum::{Json, extract::State};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    error::{Error, Result},
    feature::auth::extractor::AccountID,
    shared::ApiState,
};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(as = payment_test::create_link::Request)]
pub struct Request {
    #[schema(example = 1000)]
    pub amount: u32,
}

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    post,
    operation_id = "payment_test::create_link",
    tag = "Test",
    path = "/payment-test",
    request_body(
        content = Request,
        description = "Create a payment link"
    ),
    security(("jwt_token" = [])),
    responses(
        (
            status = 200,
            description = "Payment link created successfully",
            body = String,
            example = json!("https://pay.example.com/checkout/abc123")
        ),
        (
            status = 400,
            description = "Invalid payment data",
            body = Error
        ),
        (
            status = 401,
            description = "Unauthorized",
            body = Error
        ),
        (
            status = 500,
            description = "Internal server error",
            body = Error
        )
    )
)]
pub async fn create_link(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Json(req): Json<Request>,
) -> Result<String> {
    state
        .payment_test_svc
        .create_link(&state.payos, account_id, req.amount)
        .await
}
