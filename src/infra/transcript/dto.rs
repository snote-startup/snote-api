use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum SpeechModel {
    #[serde(rename = "universal-3-pro")]
    Universal3Pro,

    #[serde(rename = "universal-2")]
    Universal2,
}

#[derive(Serialize)]
pub struct CreateTranscriptRequest<'a> {
    pub audio_url: &'a str,
    pub speech_models: &'a [SpeechModel],
    pub speaker_labels: bool,
    pub language_detection: bool,
}

#[derive(Deserialize)]
pub struct CreateTranscriptResponse {
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptStatus {
    Processing,
    Completed,
    Error,
}

#[derive(Deserialize)]
pub struct Segment {
    pub speaker: String,
    pub text: String,
    pub start: i32,
    pub end: i32,
}

#[derive(Deserialize)]
pub struct GetTranscriptResponse {
    pub status: TranscriptStatus,
    pub utterances: Option<Vec<Segment>>,
    pub error: Option<String>,
}
