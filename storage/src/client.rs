use chezmoi_entity::CowStr;
use sqlx::migrate::Migrator;
use sqlx::Executor;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations/sqlite");

#[derive(Debug)]
pub struct Config {
    pub url: CowStr<'static>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: CowStr::Borrowed(":memory:"),
        }
    }
}

impl Config {
    pub fn new(url: impl Into<CowStr<'static>>) -> Self {
        Self { url: url.into() }
    }

    pub async fn build(&self) -> sqlx::Result<Client> {
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
            .await?;
        Ok(Client { inner })
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    inner: sqlx::SqlitePool,
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
        let client = Config::default().build().await.unwrap();
        client.upgrade().await.unwrap();
        client
    }
}
