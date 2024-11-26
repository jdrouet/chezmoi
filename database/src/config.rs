use std::borrow::Cow;

use anyhow::Context;
use chezmoi_helper::env::from_env_or;

#[derive(Clone, Debug)]
pub struct Config {
    url: Cow<'static, str>,
}

impl Config {
    pub fn memory() -> Self {
        Self {
            url: ":memory:".into(),
        }
    }

    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self { url: url.into() }
    }

    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            url: from_env_or("DATABASE_URL", ":memory:"),
        })
    }

    pub async fn build(self) -> anyhow::Result<crate::Client> {
        let opts = sqlx::sqlite::SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(self.url.as_ref());
        let inner = sqlx::sqlite::SqlitePoolOptions::new()
            // we need at least 1 connection, otherwise it looses the data when using in memory db
            .min_connections(1)
            .max_connections(1)
            .idle_timeout(None)
            .max_lifetime(None)
            .connect_with(opts)
            .await
            .context("building connection pool")?;
        Ok(crate::Client { inner })
    }
}
