use std::sync::Arc;

use axum::{Json, extract::State};
use http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    feature::project,
    transport::http::{
        error::{ApiError, ApiResult, ResultExt},
        extractor::AccountID,
        state::ApiState,
    },
};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(as = project::create::Request)]
pub struct Request {
    #[schema(example = "Personal Portfolio")]
    pub title: String,

    #[schema(example = "My portfolio website built with Rust and Axum")]
    pub description: Option<String>,
}

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    post,
    operation_id = "project::create",
    tag = "Project",
    path = "/project",
    security(("jwt_token" = [])),
    request_body(
        content = Request,
        description = "Create a new project"
    ),
    responses(
        (
            status = 201,
            description = "Project created successfully. Returns project id.",
            body = Uuid,
            example = json!("550e8400-e29b-41d4-a716-446655440000")
        ),
        (
            status = 400,
            description = "Invalid project data",
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
pub async fn create(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Json(req): Json<Request>,
) -> ApiResult<(StatusCode, Json<Uuid>)> {
    let id = project::service::create(
        &state.database,
        account_id,
        &req.title,
        req.description.as_deref(),
    )
    .await
    .with_context(StatusCode::BAD_REQUEST, "Invalid project data")?;

    Ok((StatusCode::CREATED, Json(id)))
}
