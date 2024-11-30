use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use chezmoi_database::metrics::entity::{Metric, MetricValue};
use tokio::sync::mpsc::Sender;

const ONE_HOUR: u64 = 60 * 60;
// const ONE_DAY: u64 = ONE_HOUR * 24;

#[cfg(feature = "sensor-bt-scanner")]
pub mod bt_scanner;
#[cfg(feature = "sensor-miflora")]
pub mod miflora;
pub mod system;

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

    pub fn flush(&mut self) -> Option<Vec<Metric>> {
        if self.inner.is_empty() {
            None
        } else {
            let mut new = Vec::with_capacity(self.inner.len());
            std::mem::swap(&mut self.inner, &mut new);
            Some(new)
        }
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

    pub async fn send_all(&self, events: Option<Vec<Metric>>) {
        if let Some(events) = events {
            let count = events.len();
            if let Err(err) = self.sender.send(events).await {
                tracing::error!(message = "unable to send collected metrics", metrics = count, cause = %err);
            } else {
                tracing::debug!(message = "events collected", count = count);
            }
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
