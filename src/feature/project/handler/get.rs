use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    feature::{auth::extractor::AccountID, project::model::Project},
    shared::ApiState,
};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    get,
    operation_id = "project::get",
    tag = "Project",
    path = "/project/{id}",
    params(
        (
            "id" = Uuid,
            Path,
            description = "Project id",
            example = "550e8400-e29b-41d4-a716-446655440000"
        )
    ),
    security(("jwt_token" = [])),
    responses(
        (status = 200, body = Project),
        (status = 404, description = "Project not found", body = Error),
        (
            status = 400,
            description = "Invalid project id",
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
pub async fn get(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Path(id): Path<Uuid>,
) -> Result<Json<Project>> {
    state
        .project_svc
        .get(&state.db, account_id, id)
        .await
        .map(Json)
}
