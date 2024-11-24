use std::borrow::Cow;
use std::sync::Arc;

pub mod aggr;
pub mod entity;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetricHeader {
    pub name: MetricName,
    #[serde(skip_serializing_if = "MetricTags::is_empty")]
    pub tags: MetricTags,
}

pub struct Create<'a>(&'a [entity::Metric]);

impl<'a> Create<'a> {
    #[inline]
    pub fn new(list: &'a [entity::Metric]) -> Self {
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MetricName(pub Cow<'static, str>);

impl MetricName {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self(name.into())
    }
}

impl AsRef<str> for MetricName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::fmt::Display for MetricName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MetricTagValue {
    Text(Cow<'static, str>),
    ArcText(Arc<String>),
    Float(f64),
    Int(i64),
    Boolean(bool),
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MetricTags(pub indexmap::IndexMap<Cow<'static, str>, MetricTagValue>);

impl MetricTags {
    #[inline]
    pub fn set(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        value: MetricTagValue,
    ) -> Option<MetricTagValue> {
        self.0.insert(name.into(), value)
    }

    pub fn with(mut self, name: impl Into<Cow<'static, str>>, value: MetricTagValue) -> Self {
        self.0.insert(name.into(), value);
        self
    }

    pub fn maybe_with(
        mut self,
        name: impl Into<Cow<'static, str>>,
        value: Option<MetricTagValue>,
    ) -> Self {
        if let Some(value) = value {
            self.0.insert(name.into(), value);
        }
        self
    }

    pub fn entries(&self) -> impl Iterator<Item = (&Cow<'static, str>, &MetricTagValue)> {
        self.0.iter()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
