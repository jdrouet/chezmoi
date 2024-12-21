use chezmoi_entity::{metric::Metric, OneOrMany};
use tokio::sync::mpsc;

use crate::helper::cache::Cache;

pub struct CachedSender {
    cache: Cache,
    sender: mpsc::Sender<OneOrMany<Metric>>,
}

impl CachedSender {
    pub fn new(cache: Cache, sender: mpsc::Sender<OneOrMany<Metric>>) -> Self {
        Self { cache, sender }
    }

    pub async fn send_one(&mut self, item: Metric) {
        if let Some(item) = self.cache.handle(item) {
            if let Err(err) = self.sender.send(OneOrMany::One(item)).await {
                tracing::error!(message = "unable to send events", error = %err);
            }
        }
    }

    pub async fn send_many(&mut self, items: impl IntoIterator<Item = Metric>) {
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
