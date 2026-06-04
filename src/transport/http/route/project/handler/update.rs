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
        error::{ApiResult, ResultExt},
        extractor::AccountID,
        state::ApiState,
    },
};

#[derive(Deserialize, ToSchema)]
pub struct Request {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[utoipa::path(
    patch,
    operation_id = "project::update",
    tag = "Project",
    path = "/project/{id}",
    params(
        ("id" = Uuid, Path)
    ),
    security(("jwt_token" = [])),
)]
pub async fn update(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Path(id): Path<Uuid>,
    Json(req): Json<Request>,
) -> ApiResult<()> {
    project::service::update(
        &state.database,
        id,
        account_id,
        req.title.as_deref(),
        req.description.as_deref(),
    )
    .await
    .with_context(StatusCode::BAD_REQUEST, "Invalid title or description")
}
