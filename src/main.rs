use std::env;

use opentelemetry::global;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod bootstrap;
mod doc;
mod error;
mod feature;
mod infra;
mod shared;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    if env::var("NO_COLOR") == Err(env::VarError::NotPresent) {
        color_eyre::install()?;
    } else {
        color_eyre::config::HookBuilder::new()
            .theme(color_eyre::config::Theme::new())
            .install()?;
    }

    let otlp_exporter = SpanExporter::builder().with_tonic().build()?;

    let resource = opentelemetry_sdk::Resource::builder().build();

    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .with_resource(resource)
        .build();

    global::set_tracer_provider(tracer_provider);

    let tracer = global::tracer("api-tracer");
    let otel_layer = OpenTelemetryLayer::new(tracer)
        .with_threads(false)
        .with_target(false)
        .with_tracked_inactivity(false);

    tracing_subscriber::registry()
        .with(ErrorLayer::default())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer().with_thread_ids(false))
        .with(otel_layer)
        .init();

    bootstrap::run().await
}
