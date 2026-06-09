use serde::Deserialize;
use uuid::Uuid;

pub struct TranscriptSegment {
    pub id: Uuid,
    pub speaker: String,
    pub text: String,
    pub start: i32,
    pub end: i32,
}
