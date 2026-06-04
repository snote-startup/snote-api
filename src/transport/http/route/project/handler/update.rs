use std::sync::Arc;

use axum::{
    Json,
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
        extractor::AccountID,
        state::ApiState,
    },
};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(as = project::update::Request)]
pub struct Request {
    #[schema(example = "Updated Portfolio Website")]
    pub title: Option<String>,

    #[schema(example = "Updated project description")]
    pub description: Option<String>,
}

#[utoipa::path(
    patch,
    operation_id = "project::update",
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
    request_body(
        content = Request,
        description = "Fields to update. Omitted fields are left unchanged."
    ),
    responses(
        (
            status = 204,
            description = "Project updated successfully"
        ),
        (
            status = 400,
            description = "Invalid project id, title, or description",
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
pub async fn update(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Path(id): Path<Uuid>,
    Json(req): Json<Request>,
) -> ApiResult<StatusCode> {
    project::service::update(
        &state.database,
        id,
        account_id,
        req.title.as_deref(),
        req.description.as_deref(),
    )
    .await
    .with_context(StatusCode::BAD_REQUEST, "Invalid title or description")?;

    Ok(StatusCode::NO_CONTENT)
}
