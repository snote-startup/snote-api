use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, sqlx::Type, ToSchema)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ChatRole {
    User,
    Assistant,
}

#[derive(Serialize, ToSchema)]
pub struct ChatMessage {
    pub id: Uuid,
    pub role: ChatRole,
    pub content: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessageCursor {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}

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
