mod dto;

use std::time::Duration;

use http::header::AUTHORIZATION;
use tokio::time::sleep;

use crate::error::Result;

pub use dto::Segment;
use dto::{
    CreateTranscriptRequest, CreateTranscriptResponse, GetTranscriptResponse, SpeechModel,
    TranscriptStatus,
};

const ASSEMBLY_AI_URL: &str = "https://api.assemblyai.com";
const POLL_INTERVAL: Duration = Duration::from_secs(1);

pub struct AssemblyAIClient {
    pub api_key: String,
}

impl AssemblyAIClient {
    #[tracing::instrument(err(Debug), skip(self))]
    pub async fn create_transcript(&self, audio_url: &str) -> Result<String> {
        let url = format!("{}/v2/transcript", ASSEMBLY_AI_URL);
        let req = CreateTranscriptRequest {
            audio_url,
            speech_models: &[SpeechModel::Universal3Pro, SpeechModel::Universal2],
            speaker_labels: true,
            language_detection: true,
        };
        let resp: CreateTranscriptResponse = reqwest::Client::new()
            .post(url)
            .header(AUTHORIZATION, &self.api_key)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp.id)
    }

    #[tracing::instrument(err(Debug), skip(self))]
    pub async fn get_transcript(&self, id: &str) -> Result<Vec<Segment>> {
        let url = format!("{}/v2/transcript/{}", ASSEMBLY_AI_URL, id);
        loop {
            let resp: GetTranscriptResponse = reqwest::Client::new()
                .get(&url)
                .header(AUTHORIZATION, &self.api_key)
                .send()
                .await?
                .json()
                .await?;
            match resp.status {
                TranscriptStatus::Processing => sleep(POLL_INTERVAL).await,
                TranscriptStatus::Completed => return Ok(resp.utterances.unwrap()),
                TranscriptStatus::Error => {
                    return Err(color_eyre::eyre::eyre!(resp.error.unwrap()).into());
                }
            }
        }
    }
}
