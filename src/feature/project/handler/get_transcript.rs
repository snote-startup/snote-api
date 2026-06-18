use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    feature::{auth::extractor::AccountID, project::model::TranscriptSegment},
    shared::ApiState,
};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    get,
    operation_id = "project::get_transcript",
    tags = ["Transcript", "Project"],
    path = "/project/{id}/transcript",
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
            body = Vec<TranscriptSegment>
        ),
        (
            status = 400,
            description = "Project with given id does not exist",
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
pub async fn get_transcript(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<TranscriptSegment>>> {
    state
        .project_svc
        .get_transcript(&state.db, account_id, id)
        .await
        .map(Json)
}
