use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::Redirect,
};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{error::Result, shared::ApiState};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(as = payment_test::handle_return::Params)]
pub struct Params {
    pub order_code: i64,
}

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    get,
    operation_id = "quota::handle_payment_return",
    tag = "Quota",
    path = "/quota/payment/return"
)]
pub async fn handle_payment_return(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<Params>,
) -> Result<Redirect> {
    state
        .quota_svc
        .handle_payment_return(&state.db, &state.payos, params.order_code)
        .await?;

    Ok(Redirect::to(&state.quota_svc.redirect_url))
}
