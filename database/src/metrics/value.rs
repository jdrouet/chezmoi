#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum MetricValueKind {
    Count,
}

impl std::fmt::Display for MetricValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl MetricValueKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Count => "count",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum MetricValue {
    Count { value: u64 },
}

impl std::fmt::Display for MetricValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Count { value } => write!(f, "{value} # count"),
        }
    }
}

impl MetricValue {
    pub const fn kind(&self) -> MetricValueKind {
        match self {
            Self::Count { .. } => MetricValueKind::Count,
        }
    }

    pub const fn as_count_value(&self) -> Option<u64> {
        match self {
            Self::Count { value } => Some(*value),
        }
    }
}
