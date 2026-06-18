use std::sync::Arc;

use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{error::Error, feature::auth::extractor::AccountID, shared::ApiState};

#[utoipa::path(
    post,
    operation_id = "project::stream_audio",
    tag = "Project",
    path = "/project/{id}/stream",
    params(
        (
            "id" = Uuid,
            Path,
            description = "Project id",
            example = "550e8400-e29b-41d4-a716-446655440000"
        )
    ),
    security(("jwt_token" = [])),
    responses(
        (
            status = 101,
            description = "Switches to a WebSocket connection for audio streaming"
        ),
        (
            status = 400,
            description = "Invalid project id or upgrade request",
            body = Error
        ),
        (
            status = 401,
            description = "Unauthorized",
            body = Error
        ),
        (
            status = 404,
            description = "Project not found",
            body = Error
        ),
        (
            status = 500,
            description = "Internal server error",
            body = Error
        )
    )
)]
pub async fn stream_audio(
    State(state): State<Arc<ApiState>>,
    _: AccountID,
    Path(id): Path<Uuid>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        let _ = state
            .project_svc
            .stream_audio(&state.db, &state.s3, id, socket)
            .await;
    })
}
