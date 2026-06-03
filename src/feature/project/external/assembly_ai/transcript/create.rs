use serde::{Deserialize, Serialize};

use crate::feature::project::{
    external::assembly_ai::{API_URL, SpeechModel},
    model::Transcript,
};

#[derive(Serialize)]
struct Request {
    pub audio_url: String,
    pub speech_models: Vec<SpeechModel>,
    pub speaker_labels: bool,
    pub language_detections: bool,
}

#[derive(Deserialize)]
pub struct Response {
    pub id: String,
    #[serde(rename = "utterances")]
    pub transcripts: Vec<Transcript>,
}

pub async fn create(audio_url: String) -> color_eyre::Result<Response> {
    let url = format!("{}/v2/transcript", API_URL);

    let req = Request {
        audio_url,
        speech_models: vec![SpeechModel::Universal3Pro, SpeechModel::Universal2],
        speaker_labels: true,
        language_detections: true,
    };

    let resp: Response = reqwest::Client::new()
        .post(url)
        .json(&req)
        .send()
        .await?
        .json()
        .await?;

    Ok(resp)
}
