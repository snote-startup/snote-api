use std::{sync::LazyLock, time::Duration};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    pub database_url: String,

    #[serde(default = "default_origins")]
    pub origins: String,

    #[serde(default = "default_bcrypt_cost")]
    pub bcrypt_cost: u32,

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

    pub assembly_ai_api_key: String,

    pub gemini_api_key: String,
    #[serde(default = "default_chat_context_history_size")]
    pub chat_context_history_size: usize,
    #[serde(default = "default_chat_context_transcript_size")]
    pub chat_context_trascript_size: usize,
}

#[allow(unused)]
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    ::config::Config::builder()
        .add_source(
            ::config::Environment::default()
                .try_parsing(true)
                .separator("__"),
        )
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
});

const fn default_port() -> u16 {
    3000
}

fn default_origins() -> String {
    "http://localhost:3000".to_string()
}

const fn default_bcrypt_cost() -> u32 {
    10
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

const fn default_chat_context_history_size() -> usize {
    5
}

const fn default_chat_context_transcript_size() -> usize {
    10
}
