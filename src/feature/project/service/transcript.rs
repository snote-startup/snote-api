use std::time::Duration;

use aws_sdk_s3::primitives::ByteStream;
use http::header::AUTHORIZATION;
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use tokio::time::sleep;
use uuid::Uuid;

use crate::{
    config::Config,
    error::Result,
    feature::project::{model::TranscriptSegment, repository},
    state::AppState,
};

const ASSEMBLY_AI_URL: &str = "https://api.assemblyai.com";
const POLL_INTERVAL: Duration = Duration::from_secs(3);

#[tracing::instrument(err(Debug), skip(database))]
pub async fn get_transcript(
    AppState { database, .. }: &AppState,

    account_id: Uuid,
    id: Uuid,
) -> Result<Vec<TranscriptSegment>> {
    let transcripts = repository::get_transcript(database, id).await?;

    Ok(transcripts)
}

#[tracing::instrument(err(Debug), skip(database, storage_service, config))]
pub async fn create_transcript(
    AppState {
        database,
        storage_service,
        config,
        ..
    }: &AppState,

    account_id: Uuid,
    id: Uuid,
    content: ByteStream,
) -> Result<()> {
    let key = format!("{}/audio", id);
    let audio_url = storage_service.upload(key, content).await?;

    let transcript_ai_id = create_assembly_ai_transcript(config, &audio_url).await?;
    let transcript = get_assembly_ai_transcript(config, &transcript_ai_id).await?;
    let speakers: Vec<_> = transcript.iter().map(|x| x.speaker.clone()).collect();
    let texts: Vec<_> = transcript.iter().map(|x| x.text.clone()).collect();
    let starts: Vec<_> = transcript.iter().map(|x| x.start).collect();
    let ends: Vec<_> = transcript.iter().map(|x| x.end).collect();

    let mut transaction = database.begin().await?;

    repository::update_project(
        &mut *transaction,
        id,
        account_id,
        None,
        None,
        Some(&audio_url),
        Some(&transcript_ai_id),
    )
    .await?;
    repository::create_transcript(&mut *transaction, id, &speakers, &texts, &starts, &ends);

    transaction.commit().await?;

    Ok(())
}

async fn create_assembly_ai_transcript(
    config: &Config,

    audio_url: &str,
) -> color_eyre::Result<String> {
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
        .header(AUTHORIZATION, &config.assembly_ai_api_key)
        .json(&req)
        .send()
        .await?
        .json()
        .await?;

    Ok(resp.id)
}

async fn get_assembly_ai_transcript(
    config: &Config,

    id: &str,
) -> color_eyre::Result<Vec<TranscriptSegment>> {
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
        pub utterances: Option<Vec<TranscriptSegment>>,
        pub error: Option<String>,
    }

    let url = format!("{}/v2/transcript/{}", ASSEMBLY_AI_URL, id);
    loop {
        let resp: Response = reqwest::Client::new()
            .get(&url)
            .header(AUTHORIZATION, &config.assembly_ai_api_key)
            .send()
            .await?
            .json()
            .await?;
        match resp.status {
            Status::Processing => sleep(POLL_INTERVAL).await,
            Status::Completed => return Ok(resp.utterances.unwrap()),
            Status::Error => return Err(color_eyre::eyre::anyhow!(resp.error.unwrap())),
        }
    }
}
