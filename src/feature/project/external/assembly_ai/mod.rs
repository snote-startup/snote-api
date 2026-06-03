pub mod transcript;

use serde::Serialize;

const API_URL: &str = "https://api.assemblyai.com";

#[derive(Serialize)]
pub enum SpeechModel {
    #[serde(rename = "universal-3-pro")]
    Universal3Pro,

    #[serde(rename = "universal-2")]
    Universal2,
}
