use chezmoi_agent::collector::prelude::{Collector, Context};
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

    let ctx = Context::new(sender);

    let mut jobs = Vec::with_capacity(1);

    let internal_ctx = ctx.clone();
    jobs.push(tokio::spawn(async move {
        let mut col = collector::internal::Config::default().build();
        col.run(internal_ctx).await
    }));

    exporter::Exporter::default()
        .with_flush_capacity(20)
        .run(receiver)
        .await;
    Ok(())
}
