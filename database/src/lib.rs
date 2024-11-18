pub mod config;
pub mod helper;
pub mod metrics;

pub use sqlx;
use sqlx::migrate::Migrator;
use sqlx::Executor;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations/sqlite");

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
