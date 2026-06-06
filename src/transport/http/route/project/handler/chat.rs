use std::sync::Arc;

use axum::{
    Json,
    body::Body,
    extract::{Path, State},
};
use http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    feature::project,
    transport::http::{
        error::{ApiError, ApiResult, ResultExt},
        state::ApiState,
    },
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
    tag = "Project",
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
pub async fn chat(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<Request>,
) -> ApiResult<Body> {
    let stream = project::service::chat(state.database.clone(), id, req.prompt)
        .await
        .with_context(StatusCode::BAD_REQUEST, "Invalid prompt")?;

    Ok(Body::from_stream(stream))
}
