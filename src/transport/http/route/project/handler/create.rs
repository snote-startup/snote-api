use std::sync::Arc;

use axum::{Json, extract::State};
use http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    feature::project,
    transport::http::{
        error::{ApiResult, ResultExt},
        extractor::AccountID,
        state::ApiState,
    },
};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(as = project::create::Request)]
pub struct Request {
    pub title: String,
    pub description: Option<String>,
}

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    post,
    operation_id = "project::create",
    tag = "Project",
    path = "/project",
    security(("jwt_token" = [])),
    request_body(content = Request),
)]
pub async fn create(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Json(req): Json<Request>,
) -> ApiResult<Json<Uuid>> {
    project::service::create(
        &state.database,
        account_id,
        &req.title,
        req.description.as_deref(),
    )
    .await
    .map(Json)
    .with_context(StatusCode::BAD_REQUEST, "Invalid project data")
}
