use std::sync::Arc;

use axum::extract::{Path, State};
use http::StatusCode;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    feature::auth::extractor::AccountID,
    shared::ApiState,
};

#[allow(unused)]
#[derive(Debug, ToSchema)]
#[schema(as = project::create_transcript::Request)]
pub struct Request {
    #[schema(
        value_type = String,
        format = Binary,
    )]
    pub audio: String,
}

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    post,
    operation_id = "project::create_transcript",
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
    request_body(
        content = Request,
        content_type = "multipart/form-data",
        description = "Upload an audio file"
    ),
    security(("jwt_token" = [])),
    responses(
        (
            status = 204,
            description = "Audio uploaded successfully"
        ),
        (
            status = 400,
            description = "Payload is not multipart, no audio file provided, or invalid project id",
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
// TODO: check file type
pub async fn create_transcript(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    state
        .project_svc
        .create_transcript(&state.db, &state.assembly_ai, account_id, id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
