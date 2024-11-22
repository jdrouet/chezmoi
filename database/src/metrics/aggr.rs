use sqlx::types::Json;

use crate::metrics::{MetricHeader, MetricName, MetricTagValue, MetricTags};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeRange {
    pub from: u64,
    pub to: u64,
    pub count: u64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum MetricValueAggr {
    Count {
        min: u64,
        avg: f64,
        max: u64,
        sum: u64,
    },
    Gauge {
        min: f64,
        avg: f64,
        max: f64,
    },
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetricAggr {
    pub timerange: TimeRange,
    #[serde(flatten)]
    pub header: MetricHeader,
    pub value: MetricValueAggr,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for MetricAggr {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        let Json(timerange): Json<TimeRange> = row.try_get(0)?;
        let metric_name: String = row.try_get(1)?;
        let Json(metric_tags): Json<MetricTags> = row.try_get(2)?;
        let Json(metric_value): Json<MetricValueAggr> = row.try_get(3)?;

        Ok(Self {
            timerange,
            header: MetricHeader {
                name: MetricName(metric_name.into()),
                tags: metric_tags,
            },
            value: metric_value,
        })
    }
}

macro_rules! tag_value_bind {
    ($qb:ident, $value:ident) => {
        match $value {
            MetricTagValue::Text(inner) => $qb.push_bind(inner.as_ref()),
            MetricTagValue::Float(inner) => $qb.push_bind(inner),
            MetricTagValue::Int(inner) => $qb.push_bind(inner),
            MetricTagValue::Boolean(inner) => $qb.push_bind(inner),
        }
    };
}

pub struct ListAggregation<'a> {
    headers: &'a [MetricHeader],
    timerange: (u64, u64),
    count: usize,
}

impl<'a> ListAggregation<'a> {
    pub fn new(headers: &'a [MetricHeader], timerange: (u64, u64), count: usize) -> Self {
        Self {
            headers,
            timerange,
            count,
        }
    }

    fn build_subset_headers_filter<'b>(
        &self,
        qb: &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>,
    ) -> &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>
    where
        'a: 'b,
    {
        if self.headers.is_empty() {
            qb
        } else {
            qb.push(" and (");
            for (index, header) in self.headers.iter().enumerate() {
                if index > 0 {
                    qb.push(" or");
                }
                qb.push(" (name = ").push_bind(header.name.as_ref());
                for (name, value) in header.tags.entries() {
                    qb.push(" and json_extract(tags, ")
                        .push_bind(name)
                        .push(") = ");
                    tag_value_bind!(qb, value);
                }
                qb.push(")");
            }
            qb.push(")")
        }
    }

    fn build_subset<'b>(
        &self,
        qb: &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>,
    ) -> &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>
    where
        'a: 'b,
    {
        let (from_ts, to_ts) = self.timerange;
        qb.push("select");
        qb.push(" timestamp,");
        qb.push(" (timestamp - ")
            .push_bind(from_ts as i64)
            .push(") * ")
            .push_bind(self.count as i64)
            .push(" / (")
            .push_bind(to_ts as i64)
            .push(" - ")
            .push_bind(from_ts as i64)
            .push(")")
            .push(" as section,");
        qb.push(" name,");
        qb.push(" tags,");
        qb.push(" value");
        qb.push(" from metrics");
        qb.push(" where timestamp > ").push(self.timerange.0);
        qb.push(" and timestamp <= ").push(self.timerange.1);
        self.build_subset_headers_filter(qb)
    }

    fn build_count_subset<'b>(
        &self,
        qb: &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>,
    ) -> &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>
    where
        'a: 'b,
    {
        qb.push(" select");
        qb.push(" json_object('from', min(timestamp), 'to', max(timestamp), 'count', count(timestamp)) as timestamp,");
        qb.push(" name, tags,");
        qb
            .push(" json_object('type', 'count', 'min', min(json_extract(value, '$.value')), 'avg', avg(json_extract(value, '$.value')), 'max', max(json_extract(value, '$.value')), 'sum', sum(json_extract(value, '$.value'))) as value");
        qb.push(" from metrics_subset");
        qb.push(" where json_extract(value, '$.type') = 'count'");
        qb.push(" group by section, name, tags")
    }

    fn build_gauge_subset<'b>(
        &self,
        qb: &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>,
    ) -> &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>
    where
        'a: 'b,
    {
        qb.push(" select");
        qb.push(" json_object('from', min(timestamp), 'to', max(timestamp), 'count', count(timestamp)) as timestamp,");
        qb.push(" name, tags,");
        qb
            .push(" json_object('type', 'gauge', 'min', min(json_extract(value, '$.value')), 'avg', avg(json_extract(value, '$.value')), 'max', max(json_extract(value, '$.value'))) as value");
        qb.push(" from metrics_subset");
        qb.push(" where json_extract(value, '$.type') = 'gauge'");
        qb.push(" group by section, name, tags")
    }

    pub async fn execute<'c, E: sqlx::Executor<'c, Database = sqlx::Sqlite>>(
        self,
        executor: E,
    ) -> sqlx::Result<Vec<MetricAggr>> {
        // metrics_subset
        let mut qb = sqlx::QueryBuilder::new("with metrics_subset as (");
        let qb = self.build_subset(&mut qb);
        qb.push(")");
        // count metrics
        qb.push(", metrics_count_subset as (");
        let qb = self.build_count_subset(qb);
        qb.push(")");
        // gauge metrics
        qb.push(", metrics_gauge_subset as (");
        let qb = self.build_gauge_subset(qb);
        qb.push(")");
        // main query
        qb.push(" select * from metrics_count_subset");
        qb.push(" union select * from metrics_gauge_subset");
        //
        let query = qb.build_query_as::<'_, MetricAggr>();
        let rows = query.fetch_all(executor).await?;
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::now;
    use crate::metrics::{MetricHeader, MetricName, MetricTagValue, MetricTags};

    #[tokio::test]
    async fn should_build_query() {
        let current = now();
        let before = current - 60 * 60; // 1h gap
        let headers = vec![MetricHeader {
            name: MetricName::new("hello.world"),
            tags: MetricTags::default().with("host", MetricTagValue::Text("whatever".into())),
        }];
        let client = crate::config::Config::new(":memory:")
            .build()
            .await
            .unwrap();
        client.upgrade().await.unwrap();
        let list = super::ListAggregation::new(&headers, (before, current), 10)
            .execute(client.as_ref())
            .await
            .unwrap();
        assert_eq!(list.len(), 0);
    }
}
