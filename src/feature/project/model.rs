use chrono::{DateTime, Utc};

pub struct Transcript {
    pub speaker: String,
    pub content: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}
