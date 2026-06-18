use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema, Clone, Copy)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema, Clone, Copy)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

pub struct MinimalTask {
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub content: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Task {
    pub id: Uuid,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
