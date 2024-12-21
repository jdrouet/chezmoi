use std::sync::Arc;

use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc;

use indexmap::IndexMap;

use crate::metric::AgentMetric;

#[derive(Debug)]
struct CacheEntry {
    timestamp: u64,
    value: f64,
}

struct Cache {
    inner: IndexMap<u64, CacheEntry>,
    max_capacity: usize,
    ttl: u64,
}

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache")
            .field("size", &self.inner.len())
            .field("max_capacity", &self.max_capacity)
            .field("ttl", &self.ttl)
            .finish()
    }
}

impl Cache {
    fn new(max_capacity: usize, ttl: u64) -> Self {
        Self {
            inner: IndexMap::new(),
            max_capacity,
            ttl,
        }
    }

    fn handle(&mut self, metric: AgentMetric) -> Option<AgentMetric> {
        let hash = metric.header.compute_hash();
        let found = self
            .inner
            .entry(hash)
            .and_modify(|previous| {
                if previous.value != metric.value
                    || previous.timestamp + self.ttl < metric.timestamp
                {
                    previous.timestamp = metric.timestamp;
                    previous.value = metric.value;
                }
            })
            .or_insert(CacheEntry {
                timestamp: metric.timestamp,
                value: metric.value,
            });
        if found.timestamp > metric.timestamp
            || (found.timestamp == metric.timestamp && found.value == metric.value)
        {
            Some(metric)
        } else {
            None
        }
    }

    fn handle_iter<'a>(
        &'a mut self,
        iter: impl Iterator<Item = AgentMetric> + 'a,
    ) -> impl Iterator<Item = AgentMetric> + 'a {
        iter.filter_map(|item| self.handle(item))
    }
}

pub struct CachedSender {
    cache: Cache,
    sender: mpsc::Sender<OneOrMany<AgentMetric>>,
}

impl CachedSender {
    pub fn new(capacity: usize, ttl: u64, sender: mpsc::Sender<OneOrMany<AgentMetric>>) -> Self {
        Self {
            cache: Cache::new(capacity, ttl),
            sender,
        }
    }

    pub async fn send_one(&mut self, item: AgentMetric) {
        if let Some(item) = self.cache.handle(item) {
            if let Err(err) = self.sender.send(OneOrMany::One(item)).await {
                tracing::error!(message = "unable to send events", error = %err);
            }
        }
    }

    pub async fn send_many(&mut self, items: impl IntoIterator<Item = AgentMetric>) {
        let items = self
            .cache
            .handle_iter(items.into_iter())
            .collect::<Vec<_>>();
        if !items.is_empty() {
            if let Err(err) = self.sender.send(OneOrMany::Many(items)).await {
                tracing::error!(message = "unable to send events", error = %err);
            }
        }
    }

    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }
}

#[derive(Clone, Debug, Hash)]
pub struct Hostname(Arc<str>);

impl Default for Hostname {
    fn default() -> Self {
        Self(Arc::from(
            sysinfo::System::host_name()
                .or_else(|| std::env::var("HOST").ok())
                .unwrap_or("unknown".into()),
        ))
    }
}

impl serde::Serialize for Hostname {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_ref())
    }
}
