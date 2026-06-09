use std::sync::Arc;

use axum::{Json, extract::State};
use http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    feature::auth::extractor::AccountID,
    shared::ApiState,
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
pub async fn create(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Json(req): Json<Request>,
) -> Result<(StatusCode, Json<Uuid>)> {
    let id = state
        .project_service
        .create(
            &state.db,
            account_id,
            &req.title,
            req.description.as_deref(),
        )
        .await?;

    Ok((StatusCode::CREATED, Json(id)))
}
