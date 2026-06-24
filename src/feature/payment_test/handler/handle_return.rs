use std::sync::Arc;

use axum::extract::{Query, State};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    error::{Error, Result},
    shared::ApiState,
};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(as = payment_test::handle_return::Params)]
pub struct Params {
    pub order_code: i64,
}

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    get,
    operation_id = "payment_test::handle_return",
    tag = "Test",
    path = "/payment-test/return",
    params(
        (
            "order_code" = i64,
            Query,
            description = "Payment provider order code",
            example = 123456789
        )
    ),
    responses(
        (
            status = 200,
            description = "Payment return handled successfully"
        ),
        (
            status = 400,
            description = "Invalid payment return data",
            body = Error
        ),
        (
            status = 500,
            description = "Internal server error",
            body = Error
        )
    )
)]
pub async fn handle_return(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<Params>,
) -> Result<()> {
    state
        .payment_test_svc
        .handle_return(&state.payos, params.order_code)
        .await
}
