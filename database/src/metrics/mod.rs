pub mod name;
pub mod tags;
pub mod value;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Metric {
    #[serde(default = "crate::helper::now")]
    pub timestamp: u64,
    pub name: name::MetricName,
    #[serde(skip_serializing_if = "tags::MetricTags::is_empty")]
    pub tags: tags::MetricTags,
    pub value: value::MetricValue,
}

pub struct Create<'a>(&'a [Metric]);

impl<'a> Create<'a> {
    #[inline]
    pub fn new(list: &'a [Metric]) -> Self {
        Self(list)
    }

    pub async fn execute<'c, E: sqlx::Executor<'c, Database = sqlx::Sqlite>>(
        self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let mut query_builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
            "insert into metrics (timestamp, name, tags, value_kind, value_count)",
        );
        query_builder.push_values(self.0.iter(), |mut b, entry| {
            b.push_bind(entry.timestamp as i64)
                .push_bind(entry.name.as_ref())
                .push_bind(entry.tags.urlencode())
                .push_bind(entry.value.kind().as_str())
                .push_bind(entry.value.as_count_value().map(|v| v as i64));
        });
        let query = query_builder.build();
        let res = query.execute(executor).await?;
        Ok(res.rows_affected())
    }
}
