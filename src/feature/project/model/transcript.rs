use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct TranscriptSegment {
    // HACK: add default value to id to fit with assembly_ai return type
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub speaker: String,
    pub text: String,
    pub start: i32,
    pub end: i32,
}
