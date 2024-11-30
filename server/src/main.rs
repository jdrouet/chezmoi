pub(crate) mod app;
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

    let database = chezmoi_database::Config::from_env()?;
    let database = database.build().await?;
    database.upgrade().await?;

    let agent = chezmoi_agent::Config::from_env()?;
    let agent = agent.build().await?;

    let app = app::Config::from_env()?;
    let app = app.build().await?;

    tokio::try_join!(agent.run(database.clone()), app.run(database)).map(|_| ())
}
