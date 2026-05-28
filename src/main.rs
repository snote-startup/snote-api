use std::env;

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

    transport::http::run().await
}
