use sqlx::types::Json;

use crate::metrics::{MetricHeader, MetricName, MetricTags};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Metric {
    #[serde(default = "crate::helper::now")]
    pub timestamp: u64,
    #[serde(flatten)]
    pub header: MetricHeader,
    pub value: MetricValue,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Metric {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        let metric_name: String = row.try_get(1)?;
        let Json(metric_tags): Json<MetricTags> = row.try_get(2)?;
        let Json(metric_value): Json<MetricValue> = row.try_get(3)?;

        Ok(Self {
            timestamp: row.try_get(0)?,
            header: MetricHeader {
                name: MetricName(metric_name.into()),
                tags: metric_tags,
            },
            value: metric_value,
        })
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum MetricValue {
    Count { value: u64 },
    Gauge { value: f64 },
    Bool { value: bool },
}

impl std::fmt::Display for MetricValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Count { value } => write!(f, "{value} # count"),
            Self::Gauge { value } => write!(f, "{value} # gauge"),
            Self::Bool { value } => write!(f, "{value} # bool"),
        }
    }
}

impl MetricValue {
    pub const fn count(value: u64) -> Self {
        Self::Count { value }
    }

    pub const fn gauge(value: f64) -> Self {
        Self::Gauge { value }
    }

    pub const fn bool(value: bool) -> Self {
        Self::Bool { value }
    }
}

pub struct FindLatest<'a> {
    headers: &'a [MetricHeader],
}

impl<'a> FindLatest<'a> {
    pub fn new(headers: &'a [MetricHeader]) -> Self {
        Self { headers }
    }

    pub async fn execute<'c, E: sqlx::Executor<'c, Database = sqlx::Sqlite>>(
        self,
        executor: E,
    ) -> sqlx::Result<Vec<Metric>> {
        // metrics_subset
        let mut qb = sqlx::QueryBuilder::new("with metrics_subset as (");
        qb.push("select timestamp, name, tags, value,");
        qb.push(" row_number() over (partition by name, tags order by timestamp desc) as idx");
        qb.push(" from metrics");
        qb.push(")");
        qb.push(" select timestamp, name, tags, value");
        qb.push(" from metrics_subset");
        qb.push(" where idx = 1");
        if !self.headers.is_empty() {
            qb.push(" and (");
            for (index, header) in self.headers.iter().enumerate() {
                if index > 0 {
                    qb.push(" or");
                }
                qb.push(" (")
                    .push("name = ")
                    .push_bind(header.name.as_ref())
                    .push(" and ")
                    .push(" tags = ")
                    .push_bind(Json(&header.tags))
                    .push(")");
            }
            qb.push(")");
        }
        //
        let query = qb.build_query_as::<'_, Metric>();
        let rows = query.fetch_all(executor).await?;
        Ok(rows)
    }
}
