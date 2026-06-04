use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    feature::project::{self, model::Project},
    transport::http::{error::ApiResult, extractor::AccountID, state::ApiState},
};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    get,
    operation_id = "project::get_by_account",
    tag = "Project",
    path = "/project",
    security(("jwt_token" = [])),
)]
pub async fn get_by_account(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
) -> ApiResult<Json<Vec<Project>>> {
    let projects = project::service::get_by_account(&state.database, account_id).await?;

    Ok(Json(projects))
}
