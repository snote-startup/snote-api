use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use http::StatusCode;
use uuid::Uuid;

use crate::{
    feature::project::{self, model::Project},
    transport::http::{
        error::{ApiResult, ResultExt},
        extractor::AccountID,
        state::ApiState,
    },
};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    get,
    operation_id = "project::get",
    tag = "Project",
    path = "/project/{id}",
    params(
        ("id" = Uuid, Path)
    ),
    security(("jwt_token" = [])),
)]
pub async fn get(
    State(state): State<Arc<ApiState>>,
    _: AccountID,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Option<Project>>> {
    project::service::get(&state.database, id)
        .await
        .map(Json)
        .with_context(StatusCode::BAD_REQUEST, "Invalid project id")
}
