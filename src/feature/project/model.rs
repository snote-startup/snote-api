use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Transcript {
    pub speaker: String,
    pub text: String,
    pub start: i32,
    pub end: i32,
}

#[derive(Serialize, ToSchema)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub audio_url: Option<String>,
}
