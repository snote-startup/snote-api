use std::env;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod feature;
mod transport;

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
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer().with_thread_ids(false))
        .init();

    transport::http::run().await
}
