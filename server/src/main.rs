use std::path::PathBuf;

pub(crate) mod app;
mod config;
mod router;
mod service;

fn enable_tracing() {
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    if tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            // axum logs rejections from built-in extractors with the `axum::rejection`
            // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
            "chezmoi_agent=debug,chezmoi_server=debug,tower_http=debug,axum::rejection=trace".into()
        }))
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

    let root_path = PathBuf::from("./chezmoi.toml");
    let crate::config::RootConfig {
        agent,
        database,
        server,
    } = crate::config::RootConfig::from_path(&root_path)?;

    let database = database.build().await?;
    database.upgrade().await?;

    let agent = agent.build().await?;
    let app = server.build().await?;

    let (agent, app) = tokio::join!(agent.run(database.clone()), app.run(database));
    tracing::debug!("agent success={}", agent.is_ok());
    tracing::debug!("app success={}", app.is_ok());

    Ok(())
}
