use std::{sync::LazyLock, time::Duration};

use serde::Deserialize;
use serde_with::serde_as;

#[derive(Debug, Deserialize)]
#[serde_as]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    pub database_url: String,

    #[serde(default = "default_bcrypt_cost")]
    pub bcrypt_cost: u32,
    #[serde(default = "default_bcrypt_salt")]
    #[serde_as(as = "Bytes")]
    pub bcrypt_salt: [u8; 16],

    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
    #[serde(default = "default_jwt_expired_in")]
    pub jwt_expired_in: u64,
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

const fn default_bcrypt_cost() -> u32 {
    10
}

const fn default_bcrypt_salt() -> [u8; 16] {
    [0; 16]
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
