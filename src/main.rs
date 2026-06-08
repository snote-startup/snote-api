use std::env;

use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod feature;
mod infra;
mod state;
mod util;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    if env::var("NO_COLOR") == Err(env::VarError::NotPresent) {
        color_eyre::install()?;
    } else {
        color_eyre::config::HookBuilder::new()
            .theme(color_eyre::config::Theme::new())
            .install()?;
    }

    tracing_subscriber::registry()
        .with(ErrorLayer::default())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer().with_thread_ids(false))
        .init();
}
