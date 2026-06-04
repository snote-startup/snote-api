use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use http::StatusCode;
use uuid::Uuid;

use crate::{
    feature::project::{self, model::Transcript},
    transport::http::{
        error::{ApiResult, ResultExt},
        state::ApiState,
    },
};

#[tracing::instrument(err(Debug), skip(state))]
#[utoipa::path(
    get,
    operation_id = "project::get_transcripts",
    tag = "Project",
    path = "/project/{id}/transcripts",
    params(
        ("id" = Uuid, Path)
    ),
    security(("jwt_token" = [])),
)]
pub async fn get_transcripts(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Vec<Transcript>>> {
    project::service::get_transcripts(&state.database, id)
        .await
        .map(Json)
        .with_context(StatusCode::BAD_REQUEST, "Project with given id not exist")
}
