use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use chezmoi_database::metrics::entity::{Metric, MetricValue};
use tokio::sync::mpsc::Sender;

#[cfg(feature = "bluetooth")]
pub(crate) mod bluetooth;
pub(crate) mod system;

#[derive(Clone, Debug)]
pub(crate) struct Hostname(Option<Arc<String>>);

impl Default for Hostname {
    fn default() -> Self {
        Self(sysinfo::System::host_name().map(Arc::new))
    }
}

impl Hostname {
    pub fn inner(&self) -> Option<Arc<String>> {
        self.0.clone()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct RunningState(Arc<AtomicBool>);

impl RunningState {
    fn new(running: bool) -> Self {
        Self(Arc::new(AtomicBool::new(running)))
    }

    fn is_running(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }

    #[allow(unused)]
    pub(crate) fn stop(&self) {
        self.0.store(false, Ordering::Relaxed);
    }
}

#[derive(Debug)]
pub(crate) struct Cache {
    inner: HashMap<u64, (u64, MetricValue)>,
    ttl: u64,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            ttl: 60 * 60, // 1h
        }
    }
}

impl Cache {
    pub fn maybe_take(&mut self, metric: Metric) -> Option<Metric> {
        let hash = metric.header.into_hash();
        if !self.has(hash, &metric) {
            self.insert(hash, &metric);
            Some(metric)
        } else {
            None
        }
    }

    fn insert(&mut self, hash: u64, metric: &Metric) {
        self.inner
            .insert(hash, (metric.timestamp, metric.value.clone()));
    }

    fn has(&self, header_hash: u64, metric: &Metric) -> bool {
        self.inner
            .get(&header_hash)
            .filter(|(ts, value)| ts + self.ttl > metric.timestamp && value.eq(&metric.value))
            .is_some()
    }
}

#[derive(Default)]
pub(crate) struct Collector {
    cache: Cache,
    // contains header hash and metric
    inner: Vec<Metric>,
}

impl Collector {
    pub fn new(cache: Cache, capacity: usize) -> Self {
        Self {
            cache,
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn collect(&mut self, metric: Metric) {
        if let Some(value) = self.cache.maybe_take(metric) {
            self.inner.push(value);
        }
    }

    pub fn flush(&mut self) -> Vec<Metric> {
        let mut new = Vec::with_capacity(self.inner.len());
        std::mem::swap(&mut self.inner, &mut new);
        new
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Context {
    state: RunningState,
    sender: Sender<Vec<Metric>>,
}

impl Context {
    pub fn new(running: bool, sender: Sender<Vec<Metric>>) -> Self {
        Self {
            state: RunningState::new(running),
            sender,
        }
    }
}

#[cfg(test)]
mod tests {
    use chezmoi_database::metrics::entity::{Metric, MetricValue};
    use chezmoi_database::metrics::{MetricHeader, MetricName, MetricTags};

    #[test]
    fn should_skip_metrics_without_tags() {
        let mut col = super::Collector::default();
        col.collect(Metric {
            timestamp: 0,
            header: MetricHeader {
                name: MetricName::new("foo"),
                tags: MetricTags::default(),
            },
            value: MetricValue::count(10),
        });
        col.collect(Metric {
            timestamp: 10,
            header: MetricHeader {
                name: MetricName::new("foo"),
                tags: MetricTags::default(),
            },
            value: MetricValue::count(10),
        });
        assert_eq!(col.inner.len(), 1);
        col.collect(Metric {
            timestamp: 60 * 60 * 2,
            header: MetricHeader {
                name: MetricName::new("foo"),
                tags: MetricTags::default(),
            },
            value: MetricValue::count(10),
        });
        assert_eq!(col.inner.len(), 2);
    }
}
