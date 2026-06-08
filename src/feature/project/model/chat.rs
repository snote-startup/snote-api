use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
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
