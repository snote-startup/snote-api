use std::fmt::Display;

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

impl Display for TranscriptSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[segment_id={}]", self.id)?;
        writeln!(f, "speaker={}", self.speaker)?;
        writeln!(f, "text={}", self.text)?;
        writeln!(f, "start={}", self.start)?;
        writeln!(f, "end={}", self.end)
    }
}
