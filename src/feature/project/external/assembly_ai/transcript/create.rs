use http::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

use crate::{
    config::CONFIG,
    feature::project::external::assembly_ai::{API_URL, SpeechModel},
};

#[derive(Serialize)]
struct Request<'a> {
    pub audio_url: &'a str,
    pub speech_models: &'a [SpeechModel],
    pub speaker_labels: bool,
    pub language_detection: bool,
}

#[derive(Deserialize)]
struct Response {
    pub id: String,
}

pub async fn create(audio_url: &str) -> color_eyre::Result<String> {
    let url = format!("{}/v2/transcript", API_URL);
    let req = Request {
        audio_url,
        speech_models: &[SpeechModel::Universal3Pro, SpeechModel::Universal2],
        speaker_labels: true,
        language_detection: true,
    };
    let resp: Response = reqwest::Client::new()
        .post(url)
        .header(AUTHORIZATION, &CONFIG.assembly_ai_api_key)
        .json(&req)
        .send()
        .await?
        .json()
        .await?;

    Ok(resp.id)
}
