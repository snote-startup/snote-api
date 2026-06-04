use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use http::StatusCode;
use uuid::Uuid;

use crate::{
    feature::project::{self, model::Transcript},
    transport::http::{
        error::{ApiError, ApiResult, ResultExt},
        state::ApiState,
    },
};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    get,
    operation_id = "project::get_transcripts",
    tag = "Project",
    path = "/project/{id}/transcripts",
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
            description = "List all transcripts belonging to the project",
            body = Vec<Transcript>
        ),
        (
            status = 400,
            description = "Project with given id does not exist",
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
pub async fn get_transcripts(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Vec<Transcript>>> {
    project::service::get_transcripts(&state.database, id)
        .await
        .map(Json)
        .with_context(StatusCode::BAD_REQUEST, "Project with given id not exist")
}
