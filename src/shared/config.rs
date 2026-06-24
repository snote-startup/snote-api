use std::time::Duration;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    pub database_url: String,

    #[serde(default = "default_origins")]
    pub origins: String,

    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
    #[serde(default = "default_jwt_expired_in")]
    pub jwt_expired_in: u64,
    #[serde(default = "default_jwt_secret")]
    pub jwt_refresh_secret: String,
    #[serde(default = "default_jwt_refresh_expired_in")]
    pub jwt_refresh_expired_in: u64,

    pub s3_bucket: String,
    pub aws_endpoint_url: String,

    pub payos_client_id: String,
    pub payos_api_key: String,
    pub payos_checksum_key: String,

    pub assembly_ai_api_key: String,

    pub gemini_api_key: String,
    #[serde(default = "default_chat_context_history_size")]
    pub chat_context_history_size: u32,
    #[serde(default = "default_chat_context_transcript_size")]
    pub chat_context_transcript_size: u32,
}

impl Config {
    pub fn new() -> color_eyre::Result<Self> {
        let config = ::config::Config::builder()
            .add_source(
                ::config::Environment::default()
                    .try_parsing(true)
                    .separator("__"),
            )
            .build()?
            .try_deserialize()?;

        Ok(config)
    }
}

const fn default_port() -> u16 {
    3000
}

fn default_origins() -> String {
    "http://localhost:3000".to_string()
}

fn default_jwt_secret() -> String {
    "secret".to_string()
}

const fn default_jwt_expired_in() -> u64 {
    Duration::from_mins(5).as_secs()
}

const fn default_jwt_refresh_expired_in() -> u64 {
    Duration::from_hours(7 * 24).as_secs()
}

const fn default_chat_context_history_size() -> u32 {
    10
}

const fn default_chat_context_transcript_size() -> u32 {
    10
}
