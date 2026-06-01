use uuid::Uuid;

pub struct Transcript {
    pub speaker: String,
    pub content: String,
    pub start_time: u32,
    pub end_time: u32,
}

pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
}
