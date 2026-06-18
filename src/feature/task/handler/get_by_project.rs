use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    feature::{auth::extractor::AccountID, task::model::Task},
    shared::ApiState,
};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    get,
    operation_id = "project::get_task",
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
            status = 200,
            description = "List all tasks belonging to the project",
            body = Vec<Task>
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
pub async fn get_by_project(
    State(state): State<Arc<ApiState>>,
    _: AccountID,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<Task>>> {
    state
        .task_svc
        .get_by_project(&state.db, project_id)
        .await
        .map(Json)
}
