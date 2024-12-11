use chezmoi_entity::metric::Metric;
use indexmap::IndexMap;

#[derive(Debug)]
struct CacheEntry {
    timestamp: u64,
    value: f64,
}

pub struct Cache {
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
    pub fn new(max_capacity: usize, ttl: u64) -> Self {
        Self {
            inner: IndexMap::new(),
            max_capacity,
            ttl,
        }
    }

    pub fn handle(&mut self, metric: Metric) -> Option<Metric> {
        let hash = metric.header.into_hash();
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

    pub fn vacuum(&mut self, now: u64) {
        if self.inner.len() > self.max_capacity {
            self.inner
                .retain(|_key, value| value.timestamp + self.ttl > now);
        }
        if self.inner.len() > self.max_capacity {
            self.inner.reverse();
            self.inner.truncate(self.max_capacity);
            self.inner.reverse();
        }
    }

    pub fn handle_iter<'a>(
        &'a mut self,
        iter: impl Iterator<Item = Metric> + 'a,
    ) -> impl Iterator<Item = Metric> + 'a {
        iter.filter_map(|item| self.handle(item))
    }
}

#[cfg(test)]
mod tests {
    use chezmoi_entity::metric::{Header, Metric};

    #[test]
    fn should_filter_with_ttl() {
        let mut cache = super::Cache::new(5, 3);
        assert!(cache
            .handle(Metric::new(0, Header::new("foo"), 0.0))
            .is_some());
        assert!(cache
            .handle(Metric::new(1, Header::new("foo"), 0.0))
            .is_none());
        assert!(cache
            .handle(Metric::new(2, Header::new("foo"), 0.0))
            .is_none());
        assert!(cache
            .handle(Metric::new(3, Header::new("foo"), 0.0))
            .is_none());
        assert!(cache
            .handle(Metric::new(4, Header::new("foo"), 0.0))
            .is_some());
    }

    #[test]
    fn should_let_older_pass() {
        let mut cache = super::Cache::new(5, 3);
        assert!(cache
            .handle(Metric::new(5, Header::new("foo"), 0.0))
            .is_some());
        assert!(cache
            .handle(Metric::new(1, Header::new("foo"), 0.0))
            .is_some());
    }

    #[test]
    fn should_let_pass_with_different_value() {
        let mut cache = super::Cache::new(5, 3);
        assert!(cache
            .handle(Metric::new(0, Header::new("foo"), 0.0))
            .is_some());
        assert!(cache
            .handle(Metric::new(1, Header::new("foo"), 0.1))
            .is_some());
    }

    #[test]
    fn should_vacuum_by_size() {
        let mut cache = super::Cache::new(5, 3);
        for c in ["a", "b", "c", "d", "e", "f", "g", "h"] {
            assert!(cache.handle(Metric::new(0, Header::new(c), 0.0)).is_some());
        }
        cache.vacuum(1);
        assert!(cache
            .handle(Metric::new(1, Header::new("a"), 0.0))
            .is_some());
    }

    #[test]
    fn should_vacuum_by_ttl() {
        let mut cache = super::Cache::new(5, 3);
        assert!(cache
            .handle(Metric::new(0, Header::new("a"), 0.0))
            .is_some());
        assert!(cache
            .handle(Metric::new(2, Header::new("b"), 0.0))
            .is_some());
        assert!(cache
            .handle(Metric::new(4, Header::new("c"), 0.0))
            .is_some());
        assert!(cache
            .handle(Metric::new(6, Header::new("d"), 0.0))
            .is_some());
        cache.vacuum(7);
        assert!(cache
            .handle(Metric::new(8, Header::new("a"), 0.0))
            .is_some());
        assert!(cache
            .handle(Metric::new(8, Header::new("b"), 0.0))
            .is_some());
        assert!(cache
            .handle(Metric::new(8, Header::new("c"), 0.0))
            .is_some());
        assert!(cache
            .handle(Metric::new(8, Header::new("d"), 0.0))
            .is_none());
    }
}
