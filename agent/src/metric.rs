use std::hash::{Hash, Hasher};

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum AgentMetricHeader {
    AtcSensor(crate::collector::atc_sensor::AgentMetricHeader),
    Internal(crate::collector::internal::AgentMetricHeader),
    MifloraSensor(crate::collector::miflora_sensor::AgentMetricHeader),
    System(crate::collector::system::AgentMetricHeader),
}

impl From<crate::collector::atc_sensor::AgentMetricHeader> for AgentMetricHeader {
    fn from(value: crate::collector::atc_sensor::AgentMetricHeader) -> Self {
        Self::AtcSensor(value)
    }
}

impl From<crate::collector::internal::AgentMetricHeader> for AgentMetricHeader {
    fn from(value: crate::collector::internal::AgentMetricHeader) -> Self {
        Self::Internal(value)
    }
}

impl From<crate::collector::miflora_sensor::AgentMetricHeader> for AgentMetricHeader {
    fn from(value: crate::collector::miflora_sensor::AgentMetricHeader) -> Self {
        Self::MifloraSensor(value)
    }
}

impl From<crate::collector::system::AgentMetricHeader> for AgentMetricHeader {
    fn from(value: crate::collector::system::AgentMetricHeader) -> Self {
        Self::System(value)
    }
}

impl AgentMetricHeader {
    pub fn compute_hash(&self) -> u64 {
        let mut h = std::hash::DefaultHasher::new();
        match self {
            Self::AtcSensor(inner) => inner.hash(&mut h),
            Self::Internal(inner) => inner.hash(&mut h),
            Self::MifloraSensor(inner) => inner.hash(&mut h),
            Self::System(inner) => inner.hash(&mut h),
        };
        h.finish()
    }
}

#[derive(Debug, serde::Serialize)]
pub struct AgentMetric {
    pub timestamp: u64,
    #[serde(flatten)]
    pub header: AgentMetricHeader,
    pub value: f64,
}

impl AgentMetric {
    pub fn new(timestamp: u64, header: impl Into<AgentMetricHeader>, value: f64) -> Self {
        Self {
            timestamp,
            header: header.into(),
            value,
        }
    }
}
