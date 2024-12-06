use sqlx::types::Json;

use crate::metrics::{MetricHeader, MetricTags};

pub mod list;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeRange {
    pub from: u64,
    pub to: u64,
    pub count: u64,
}

impl TimeRange {
    pub fn middle(&self) -> u64 {
        (self.from + self.to) / 2
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetricCountAggr {
    pub min: u64,
    pub avg: f64,
    pub max: u64,
    pub sum: u64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetricGaugeAggr {
    pub min: f64,
    pub avg: f64,
    pub max: f64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum MetricValueAggr {
    Count(MetricCountAggr),
    Gauge(MetricGaugeAggr),
}

impl MetricValueAggr {
    pub fn as_count(&self) -> Option<&MetricCountAggr> {
        match self {
            Self::Count(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn into_count(self) -> Option<MetricCountAggr> {
        match self {
            Self::Count(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_gauge(&self) -> Option<&MetricGaugeAggr> {
        match self {
            Self::Gauge(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn into_gauge(self) -> Option<MetricGaugeAggr> {
        match self {
            Self::Gauge(inner) => Some(inner),
            _ => None,
        }
    }
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
                name: metric_name.into(),
                tags: metric_tags,
            },
            value: metric_value,
        })
    }
}
