use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use http::StatusCode;
use uuid::Uuid;

use crate::{
    feature::project::{self, model::Project},
    transport::http::{
        error::{ApiError, ApiResult, Context, ResultExt},
        extractor::AccountID,
        state::ApiState,
    },
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
        (status = 404, description = "Project not found", body = ApiError),
        (
            status = 400,
            description = "Invalid project id",
            body = ApiError
        ),
        (
            status = 401,
            description = "Unauthorized",
            body = ApiError
        ),
        (
            status = 500,
            description = "Internal server error",
            body = ApiError
        )
    )
)]
pub async fn get(
    State(state): State<Arc<ApiState>>,
    _: AccountID,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Project>> {
    match project::service::get(&state.database, id)
        .await
        .with_context(StatusCode::BAD_REQUEST, "Invalid project id")?
    {
        Some(project) => Ok(Json(project)),
        None => Err(Context {
            status: StatusCode::NOT_FOUND,
            message: "Project not found".to_string(),
            ..Default::default()
        }
        .into()),
    }
}
