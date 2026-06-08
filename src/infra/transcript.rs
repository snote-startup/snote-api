use std::time::Duration;

use http::header::AUTHORIZATION;
use serde::Deserialize;
use serde_json::json;
use tokio::time::sleep;

use crate::error::Result;

const ASSEMBLY_AI_URL: &str = "https://api.assemblyai.com";
const POLL_INTERVAL: Duration = Duration::from_secs(1);

pub async fn create_transcript(api_key: &str, audio_url: &str) -> Result<String> {
    #[derive(Deserialize)]
    struct Response {
        pub id: String,
    }

    let url = format!("{}/v2/transcript", ASSEMBLY_AI_URL);
    let req = json!({
        "audio_url": audio_url,
        "speech_models": ["universal-3-pro", "universal-2"],
        "speaker_labels": true,
        "language_detection": true
    });
    let resp: Response = reqwest::Client::new()
        .post(url)
        .header(AUTHORIZATION, api_key)
        .json(&req)
        .send()
        .await?
        .json()
        .await?;

    Ok(resp.id)
}

#[derive(Deserialize)]
pub struct Segment {
    pub speaker: String,
    pub text: String,
    pub start: i32,
    pub end: i32,
}

pub async fn get_transcript(api_key: &str, id: &str) -> Result<Vec<Segment>> {
    #[derive(Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum Status {
        Processing,
        Completed,
        Error,
    }

    #[derive(Deserialize)]
    struct Response {
        pub status: Status,
        pub utterances: Option<Vec<Segment>>,
        pub error: Option<String>,
    }

    let url = format!("{}/v2/transcript/{}", ASSEMBLY_AI_URL, id);
    loop {
        let resp: Response = reqwest::Client::new()
            .get(&url)
            .header(AUTHORIZATION, api_key)
            .send()
            .await?
            .json()
            .await?;
        match resp.status {
            Status::Processing => sleep(POLL_INTERVAL).await,
            Status::Completed => return Ok(resp.utterances.unwrap()),
            Status::Error => return Err(color_eyre::eyre::eyre!(resp.error.unwrap()).into()),
        }
    }
}
