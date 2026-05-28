mod config;
mod feature;
mod transport;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    transport::http::run().await
}
