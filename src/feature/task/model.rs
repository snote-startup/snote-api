use chrono::{DateTime, Utc};
use schemars::JsonSchema;
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, sqlx::Type, ToSchema, Clone, Copy)]
#[sqlx(rename_all = "snake_case", type_name = "task_priority")]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreateTaskData {
    pub priority: TaskPriority,
    pub content: String,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct LLMResponse {
    pub task: Vec<CreateTaskData>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Task {
    pub id: Uuid,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
