use sqlx::types::Json;

pub mod create;
pub mod find_latest;

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

    pub const fn as_count(&self) -> Option<u64> {
        match self {
            Self::Count { value } => Some(*value),
            _ => None,
        }
    }

    pub const fn gauge(value: f64) -> Self {
        Self::Gauge { value }
    }

    pub const fn as_gauge(&self) -> Option<f64> {
        match self {
            Self::Gauge { value } => Some(*value),
            _ => None,
        }
    }

    pub const fn bool(value: bool) -> Self {
        Self::Bool { value }
    }

    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool { value } => Some(*value),
            _ => None,
        }
    }
}
