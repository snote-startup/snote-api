use std::sync::Arc;

use axum::extract::{Multipart, Path, State};
use http::StatusCode;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    feature::project,
    transport::http::{
        error::{ApiError, ApiResult, Context, ResultExt},
        extractor::AccountID,
        state::ApiState,
    },
};

#[allow(unused)]
#[derive(Debug, ToSchema)]
#[schema(as = project::upload_audio::Request)]
pub struct Request {
    #[schema(
        value_type = String,
        format = Binary,
    )]
    pub audio: String,
}

#[utoipa::path(
    patch,
    operation_id = "project::upload_audio",
    tag = "Project",
    path = "/project/{id}/upload",
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
        content_type = "multipart/form-data",
        description = "Upload an audio file"
    ),
    security(("jwt_token" = [])),
    responses(
        (
            status = 204,
            description = "Audio uploaded successfully"
        ),
        (
            status = 400,
            description = "Payload is not multipart, no audio file provided, or invalid project id",
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
// TODO: check file type
pub async fn upload_audio(
    State(state): State<Arc<ApiState>>,
    AccountID(account_id): AccountID,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> ApiResult<StatusCode> {
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

    Ok(StatusCode::NO_CONTENT)
}
