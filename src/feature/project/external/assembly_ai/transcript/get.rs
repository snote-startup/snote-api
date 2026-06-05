use std::time::Duration;

use http::header::AUTHORIZATION;
use serde::Deserialize;
use tokio::time::sleep;

use crate::{
    config::CONFIG,
    feature::project::{external::assembly_ai::API_URL, model::TranscriptSegment},
};

const POLL_INTERVAL: Duration = Duration::from_secs(3);

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

pub async fn get(id: &str) -> color_eyre::Result<Vec<TranscriptSegment>> {
    let url = format!("{}/v2/transcript/{}", API_URL, id);
    loop {
        let resp: Response = reqwest::Client::new()
            .get(&url)
            .header(AUTHORIZATION, &CONFIG.assembly_ai_api_key)
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
