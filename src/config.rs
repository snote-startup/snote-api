use std::{sync::LazyLock, time::Duration};

use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    pub database_url: String,

    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
    #[serde(default = "default_jwt_expired_in")]
    pub jwt_expired_in: usize,
    #[serde(default = "default_jwt_secret")]
    pub jwt_refresh_secret: String,
    #[serde(default = "default_jwt_refresh_expired_in")]
    pub jwt_refresh_expired_in: u64,
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

fn default_jwt_secret() -> String {
    "secret".to_string()
}

const fn default_jwt_expired_in() -> u64 {
    Duration::from_mins(5).as_secs()
}

const fn default_jwt_refresh_expired_in() -> u64 {
    Duration::from_hours(7 * 24).as_secs()
}
