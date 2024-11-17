use std::borrow::Cow;

use anyhow::Context;

#[derive(Clone, Debug)]
pub struct Config {
    url: Cow<'static, str>,
}

impl Config {
    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self { url: url.into() }
    }

    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            url: Cow::Borrowed(":memory:"),
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
