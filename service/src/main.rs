use abi::Config;
use anyhow::Result;
use service::start_server;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let filename = Path::new("./config.yml");

    let config = Config::load(filename)?;
    start_server(&config).await
}
