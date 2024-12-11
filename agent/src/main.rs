use chezmoi_agent::exporter::prelude::Exporter;
use chezmoi_agent::{collector, exporter};
use tokio::sync::mpsc;

fn enable_tracing() {
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    if tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .is_err()
    {
        tracing::warn!("tracing already set");
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    enable_tracing();

    let (sender, receiver) = mpsc::channel(200);

    let mut collectors = collector::Manager::new(sender);
    collectors.start(collector::internal::Config::default().build());

    // exporter::direct::DirectExporter::new(exporter::cache::CacheLayer::new(
    //     20,
    //     60 * 5,
    //     exporter::trace::TractHandler::default(),
    // ))
    // .run(receiver)
    // .await;

    exporter::batch::BatchExporter::new(exporter::cache::CacheLayer::new(
        20,
        60,
        exporter::http::HttpHandler::new("http://localhost:3000/api/metrics"),
    ))
    .run(receiver)
    .await;

    collectors.wait().await;

    Ok(())
}
