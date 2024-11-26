use std::collections::HashMap;

use super::{Metric, MetricValue};
use crate::metrics::MetricHeader;

pub struct MetricMap(HashMap<MetricHeader, (u64, MetricValue)>);

impl From<Vec<Metric>> for MetricMap {
    fn from(value: Vec<Metric>) -> Self {
        Self(HashMap::from_iter(value.into_iter().map(|metric| {
            (metric.header, (metric.timestamp, metric.value))
        })))
    }
}

impl MetricMap {
    pub fn remove(mut self, header: &MetricHeader) -> Option<Metric> {
        self.0
            .remove_entry(&header)
            .map(|(header, (timestamp, value))| Metric {
                timestamp,
                header,
                value,
            })
    }

    pub fn into_metrics(self) -> impl Iterator<Item = Metric> {
        self.0
            .into_iter()
            .map(|(header, (timestamp, value))| Metric {
                timestamp,
                header,
                value,
            })
    }
}
