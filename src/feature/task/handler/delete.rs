use std::sync::Arc;

use axum::extract::{Path, State};
use http::StatusCode;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    feature::auth::extractor::AccountID,
    shared::ApiState,
};

#[utoipa::path(
    delete,
    operation_id = "task::delete",
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
            description = "Task deleted successfully"
        ),
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
            status = 404,
            description = "Task not found",
            body = Error
        ),
        (
            status = 500,
            description = "Internal server error",
            body = Error
        )
    )
)]
pub async fn update(
    State(state): State<Arc<ApiState>>,
    _: AccountID,
    Path(project_id): Path<Uuid>,
) -> Result<StatusCode> {
    state.task_svc.delete(&state.db, project_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
