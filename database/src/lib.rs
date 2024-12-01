pub mod helper;
pub mod metrics;

use std::borrow::Cow;

use anyhow::Context;
use chezmoi_helper::env::from_env_or;
pub use sqlx;
use sqlx::migrate::Migrator;
use sqlx::Executor;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations/sqlite");

fn default_url() -> Cow<'static, str> {
    Cow::Borrowed(":memory:")
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_url")]
    url: Cow<'static, str>,
}

impl Default for Config {
    fn default() -> Self {
        Self { url: default_url() }
    }
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

#[derive(Clone, Debug)]
pub struct Client {
    inner: sqlx::Pool<sqlx::Sqlite>,
}

impl AsRef<sqlx::Pool<sqlx::Sqlite>> for Client {
    fn as_ref(&self) -> &sqlx::Pool<sqlx::Sqlite> {
        &self.inner
    }
}

impl Client {
    pub async fn ping(&self) -> sqlx::Result<()> {
        self.inner.execute("select 1").await?;
        Ok(())
    }

    pub async fn upgrade(&self) -> Result<(), sqlx::migrate::MigrateError> {
        MIGRATOR.run(&self.inner).await
    }
}

#[cfg(test)]
impl Client {
    pub async fn test() -> Self {
        let client = Config::memory().build().await.unwrap();
        client.upgrade().await.unwrap();
        client
    }
}
