use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
};
use http::StatusCode;
use uuid::Uuid;

use crate::{
    feature::project::{self, model::ChatMessage},
    transport::http::{
        error::{ApiError, ApiResult, ResultExt},
        state::ApiState,
    },
    util::pagination::{PaginatedVec, PaginationQuery},
};

#[utoipa::path(
    get,
    operation_id = "project::get_chat_messages",
    tag = "Project",
    path = "/project/{id}/chat",
    params(
        (
            "id" = Uuid,
            Path,
            description = "Project id",
            example = "550e8400-e29b-41d4-a716-446655440000"
        ),
        (
            "limit" = Option<u32>,
            Query,
            description = "Maximum number of messages to return",
            example = 20
        ),
        (
            "cursor" = Option<String>,
            Query,
            description = "Cursor returned from previous page",
            example = "eyJpZCI6IjU1MGU4NDAwLWUyOWItNDFkNC1hNzE2LTQ0NjY1NTQ0MDAwMCJ9"
        )
    ),
    security(("jwt_token" = [])),
    responses(
        (
            status = 200,
            description = "Paginated chat messages",
            body = PaginatedVec<ChatMessage>
        ),
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
// TODO: check permission
pub async fn get_chat_messages(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> ApiResult<Json<PaginatedVec<ChatMessage>>> {
    project::service::get_chat_messages(&state.database, id, query)
        .await
        .map(Json)
        .with_context(StatusCode::BAD_REQUEST, "Invalid project id")
}
