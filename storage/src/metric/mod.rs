use chezmoi_entity::metric::Metric;

pub async fn create<'c, E, M>(executor: E, metrics: M) -> sqlx::Result<u64>
where
    E: sqlx::Executor<'c, Database = sqlx::Sqlite>,
    M: Iterator<Item = &'c Metric>,
{
    let mut query_builder: sqlx::QueryBuilder<sqlx::Sqlite> =
        sqlx::QueryBuilder::new("insert into metrics (timestamp, name, tags, value)");
    query_builder.push_values(metrics, |mut b, entry| {
        b.push_bind(entry.timestamp as i64)
            .push_bind(entry.header.name.as_ref())
            .push_bind(sqlx::types::Json(&entry.header.tags))
            .push_bind(sqlx::types::Json(&entry.value));
    });
    let query = query_builder.build();
    let res = query.execute(executor).await?;
    Ok(res.rows_affected())
}
