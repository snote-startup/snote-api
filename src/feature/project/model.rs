use uuid::Uuid;

pub struct Transcript {
    pub speaker: String,
    pub text: String,
    pub start: i32,
    pub end: i32,
}

pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub audio_url: Option<String>,
}
