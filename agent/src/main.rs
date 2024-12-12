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

    tracing::debug!("loading configuration");
    let agent = chezmoi_agent::Config::from_path_or_env()?;
    let agent = agent.build().await?;

    tracing::info!("starting agent");
    agent.run().await;

    Ok(())
}
