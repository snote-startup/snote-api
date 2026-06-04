use std::sync::Arc;

use axum::extract::{Multipart, Path, State};
use http::StatusCode;
use uuid::Uuid;

use crate::{
    feature::project,
    transport::http::{
        error::{ApiError, ApiResult, Context, ResultExt},
        extractor::AccountID,
        state::ApiState,
    },
};

#[utoipa::path(
    patch,
    operation_id = "project::upload_audio",
    tag = "Project",
    path = "/project/{id}/upload",
    params(
        ("id" = Uuid, Path)
    ),
    security(("jwt_token" = [])),
)]
pub async fn upload_audio(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> ApiResult<()> {
    let Some(field) = multipart
        .next_field()
        .await
        .with_context(StatusCode::BAD_REQUEST, "Payload is not multipart")?
    else {
        return Err(Context {
            status: StatusCode::BAD_REQUEST,
            message: "No audio file provided".to_string(),
            ..Default::default()
        }
        .into());
    };
    let content = field.bytes().await?;

    project::service::upload_audio(&state.database, &state.s3, id, account_id, content.into())
        .await?;

    Ok(())
}
