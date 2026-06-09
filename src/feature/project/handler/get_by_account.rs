use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    error::{Error, Result},
    feature::{
        auth::extractor::AccountID,
        project::{self, model::Project},
    },
    shared::ApiState,
};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    get,
    operation_id = "project::get_by_account",
    tag = "Project",
    path = "/project",
    security(("jwt_token" = [])),
    responses(
        (
            status = 200,
            description = "List all projects owned by the authenticated account",
            body = Vec<Project>
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
pub async fn get_by_account(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
) -> Result<Json<Vec<Project>>> {
    state
        .project_service
        .get_by_account(&state.db, account_id)
        .await
        .map(Json)
}
