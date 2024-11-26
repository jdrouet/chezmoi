use sqlx::types::Json;

use crate::metrics::{MetricHeader, MetricName, MetricTags};

pub mod list;

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
