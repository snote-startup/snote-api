use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    feature::{
        auth::extractor::AccountID,
        task::model::{TaskPriority, TaskStatus},
    },
    shared::ApiState,
};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(as = task::update::Request)]
pub struct Request {
    #[schema(example = "in_progress")]
    pub status: Option<TaskStatus>,

    #[schema(example = "high")]
    pub priority: Option<TaskPriority>,

    #[schema(example = "Implement refresh token rotation and update tests")]
    pub content: Option<String>,
}

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    patch,
    operation_id = "task::update",
    tag = "Task",
    path = "/task/{id}",
    params(
        (
            "id" = Uuid,
            Path,
            description = "Task id",
            example = "550e8400-e29b-41d4-a716-446655440000"
        )
    ),
    request_body(
        content = Request,
        description = "Fields to update. Any omitted field remains unchanged."
    ),
    security(("jwt_token" = [])),
    responses(
        (
            status = 204,
            description = "Task updated successfully"
        ),
        (
            status = 400,
            description = "Invalid task id or update data",
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
    Path(id): Path<Uuid>,
    Json(req): Json<Request>,
) -> Result<StatusCode> {
    state
        .task_svc
        .update(
            &state.db,
            id,
            req.status,
            req.priority,
            req.content.as_deref(),
        )
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
