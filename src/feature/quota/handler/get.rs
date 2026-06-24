use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{error::Result, feature::auth::extractor::AccountID, shared::ApiState};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    get,
    operation_id = "quota::get",
    tag = "Quota",
    path = "/quota",
    security(("jwt_token" = [])),
)]
pub async fn get(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
) -> Result<Json<i32>> {
    state.quota_svc.get(&state.db, account_id).await.map(Json)
}
