pub mod config;
pub mod helper;
pub mod metrics;

pub use sqlx;
use sqlx::Executor;

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
}
