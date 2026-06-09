use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
};
use http::StatusCode;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    feature::{
        auth::extractor::AccountID,
        project::{self, model::ChatMessage},
    },
    shared::{
        ApiState,
        pagination::{PaginatedVec, PaginationQuery},
    },
};

#[utoipa::path(
    get,
    operation_id = "project::get_history",
    tag = "Project",
    path = "/project/{id}/chat/history",
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
pub async fn get_chat_history(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Path(id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedVec<ChatMessage>>> {
    let _ = state.project_service.get(&state.db, account_id, id).await?;

    state
        .chat_service
        .get_history(&state.db, id, query)
        .await
        .map(Json)
}
