use std::sync::LazyLock;

use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
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
