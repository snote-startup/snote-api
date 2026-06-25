use std::sync::Arc;

use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::shared::ApiState;

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
)]
pub async fn stream_audio(
    State(state): State<Arc<ApiState>>,
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
