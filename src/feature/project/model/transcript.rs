use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct TranscriptSegment {
    pub id: Uuid,
    pub speaker: String,
    pub text: String,
    pub start: i32,
    pub end: i32,
}
