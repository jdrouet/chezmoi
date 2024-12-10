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

    let mut collectors = Vec::with_capacity(1);
    collectors.push(collector::internal::Config::default().build());

    let mut jobs = Vec::from_iter(collectors.into_iter().map(|item| {
        let internal_ctx = ctx.clone();
        tokio::spawn(async move {
            let mut item = item;
            item.run(internal_ctx).await
        })
    }));

    exporter::Exporter::default()
        .with_flush_capacity(20)
        .run(receiver)
        .await;

    while let Some(job) = jobs.pop() {
        if let Err(err) = job.await {
            tracing::error!(message = "unable to wait for job", error = %err);
        }
    }

    Ok(())
}
