#![allow(unused)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub audio_url: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TranscriptSegment {
    pub id: Uuid,
    pub speaker: String,
    pub text: String,
    pub start: i32,
    pub end: i32,
}

#[derive(Serialize, sqlx::Type, ToSchema)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ChatRole {
    User,
    Assistant,
}

pub struct ChatMessage {
    pub id: Uuid,
    pub role: ChatRole,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessageCursor {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}
