use chezmoi_cache::Cache;
use chezmoi_entity::metric::Metric;

use super::batch::BatchHandler;
use super::direct::DirectHandler;
use crate::collector::prelude::OneOrMany;

pub struct CacheLayer<H> {
    cache: Cache,
    handler: H,
}

impl<H> CacheLayer<H> {
    pub fn new(cache_size: usize, cache_ttl: u64, handler: H) -> Self {
        Self {
            cache: Cache::new(cache_size, cache_ttl),
            handler,
        }
    }
}

impl<H: BatchHandler + Send> BatchHandler for CacheLayer<H> {
    #[tracing::instrument(name = "cache", skip_all)]
    async fn handle(&mut self, values: Vec<Metric>) {
        let values: Vec<_> = self.cache.handle_iter(values.into_iter()).collect();
        if !values.is_empty() {
            self.handler.handle(values).await;
        }
    }
}

impl<H: DirectHandler + Send> DirectHandler for CacheLayer<H> {
    #[tracing::instrument(name = "cache", skip_all)]
    async fn handle(&mut self, values: OneOrMany<Metric>) {
        match values {
            OneOrMany::One(item) => {
                if let Some(item) = self.cache.handle(item) {
                    self.handler.handle(OneOrMany::One(item)).await;
                }
            }
            OneOrMany::Many(values) => {
                let values: Vec<_> = self.cache.handle_iter(values.into_iter()).collect();
                if !values.is_empty() {
                    self.handler.handle(OneOrMany::Many(values)).await;
                }
            }
        }
    }
}
