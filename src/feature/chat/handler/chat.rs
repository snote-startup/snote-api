use std::sync::Arc;

use axum::{
    Json,
    body::Body,
    extract::{Path, State},
};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    feature::auth::extractor::AccountID,
    shared::ApiState,
};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(as = project::chat::Request)]
pub struct Request {
    #[schema(example = "Summarize the uploaded meeting transcript")]
    pub prompt: String,
}

#[utoipa::path(
    post,
    operation_id = "project::chat",
    tags = ["Chat", "Project"],
    path = "/project/{id}/chat",
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
        description = "Prompt sent to the AI assistant"
    ),
    security(("jwt_token" = [])),
    responses(
        (
            status = 200,
            description = "Streaming AI response",
            content_type = "text/plain"
        ),
        (
            status = 400,
            description = "Invalid project id or prompt",
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
pub async fn chat(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Path(id): Path<Uuid>,
    Json(req): Json<Request>,
) -> Result<Body> {
    let stream = state
        .chat_service
        .chat(
            state.db.to_owned(),
            &state.project_service,
            id,
            account_id,
            req.prompt,
        )
        .await?;

    Ok(Body::from_stream(stream))
}
