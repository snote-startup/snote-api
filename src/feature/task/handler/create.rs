use std::sync::Arc;

use axum::extract::{Path, State};
use http::StatusCode;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    feature::auth::extractor::AccountID,
    shared::ApiState,
};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    post,
    operation_id = "project::create_tasks",
    tags = ["Task", "Project"],
    path = "/project/{id}/task",
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
        (
            status = 204,
            description = "Tasks generated successfully from project content"
        ),
        (
            status = 400,
            description = "Invalid project id or project content cannot be processed",
            body = Error
        ),
        (
            status = 401,
            description = "Unauthorized",
            body = Error
        ),
        (
            status = 404,
            description = "Project not found",
            body = Error
        ),
        (
            status = 500,
            description = "Internal server error",
            body = Error
        )
    )
)]
pub async fn create(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Path(project_id): Path<Uuid>,
) -> Result<StatusCode> {
    state
        .task_svc
        .create(&state.db, &state.project_svc, account_id, project_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
