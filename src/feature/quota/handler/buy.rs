use std::sync::Arc;

use axum::extract::State;

use crate::{
    error::{Error, Result},
    feature::auth::extractor::AccountID,
    shared::ApiState,
};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    post,
    operation_id = "quota::buy",
    tag = "Quota",
    path = "/quota/buy",
    security(("jwt_token" = [])),
    responses(
        (
            status = 200,
            description = "Quota purchase link created successfully",
            body = String,
            example = json!("https://pay.example.com/checkout/abc123")
        ),
        (
            status = 401,
            description = "Unauthorized",
            body = Error
        ),
        (
            status = 400,
            description = "Failed to create quota purchase",
            body = Error
        ),
        (
            status = 500,
            description = "Internal server error",
            body = Error
        )
    )
)]
pub async fn buy(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
) -> Result<String> {
    state.quota_svc.buy(&state.payos, account_id).await
}
