use super::Metric;

pub struct Command<'a>(&'a [Metric]);

impl<'a> Command<'a> {
    #[inline]
    pub fn new(list: &'a [Metric]) -> Self {
        Self(list)
    }

    pub async fn execute<'c, E: sqlx::Executor<'c, Database = sqlx::Sqlite>>(
        self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let mut query_builder: sqlx::QueryBuilder<sqlx::Sqlite> =
            sqlx::QueryBuilder::new("insert into metrics (timestamp, name, tags, value)");
        query_builder.push_values(self.0.iter(), |mut b, entry| {
            b.push_bind(entry.timestamp as i64)
                .push_bind(entry.header.name.as_ref())
                .push_bind(sqlx::types::Json(&entry.header.tags))
                .push_bind(sqlx::types::Json(&entry.value));
        });
        let query = query_builder.build();
        let res = query.execute(executor).await?;
        Ok(res.rows_affected())
    }
}
