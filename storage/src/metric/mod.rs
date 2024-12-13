use chezmoi_entity::metric::{Metric, MetricHeader, MetricTags};
use sqlx::types::Json;

struct SqlxMetric(Metric);

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for SqlxMetric {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        let metric_name: String = row.try_get(1)?;
        let Json(metric_tags): Json<MetricTags> = row.try_get(2)?;

        Ok(SqlxMetric(Metric {
            timestamp: row.try_get(0)?,
            header: MetricHeader {
                name: metric_name.into(),
                tags: metric_tags,
            },
            value: row.try_get(3)?,
        }))
    }
}

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

pub async fn latest<'c, E>(
    executor: E,
    headers: &'c [MetricHeader<'c>],
    window: (u64, u64),
) -> sqlx::Result<Vec<Metric>>
where
    E: sqlx::Executor<'c, Database = sqlx::Sqlite>,
{
    let mut qb = sqlx::QueryBuilder::new("with metrics_subset as (");
    qb.push("select timestamp, name, tags, value,");
    qb.push(" row_number() over (partition by name, tags order by timestamp desc) as idx");
    qb.push(" from metrics");

    qb.push(" where timestamp >= ")
        .push_bind(window.0 as i64)
        .push(" and timestamp <= ")
        .push_bind(window.1 as i64);
    qb.push(")");
    qb.push(" select timestamp, name, tags, value");
    qb.push(" from metrics_subset");
    qb.push(" where idx = 1");
    if !headers.is_empty() {
        qb.push(" and (");
        for (index, header) in headers.iter().enumerate() {
            if index > 0 {
                qb.push(" or");
            }
            qb.push(" (")
                .push("name = ")
                .push_bind(header.name.as_ref());
            for (name, value) in header.tags.as_ref().iter() {
                qb.push(" and")
                    .push(" json_extract(tags, ")
                    .push_bind(format!("$.{name}"))
                    .push(") = ")
                    .push_bind(value);
            }
            qb.push(")");
        }
        qb.push(")");
    }
    qb.push(" order by timestamp desc");
    //
    let query = qb.build_query_as::<'_, SqlxMetric>();
    let rows = query.fetch_all(executor).await?;
    let rows = rows
        .into_iter()
        .map(|SqlxMetric(inner)| inner)
        .collect::<Vec<_>>();
    Ok(rows)
}
