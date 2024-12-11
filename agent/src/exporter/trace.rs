use chezmoi_entity::metric::Metric;

use crate::collector::prelude::OneOrMany;

#[derive(Debug, Default)]
pub struct TractHandler;

impl TractHandler {
    fn handle(&mut self, item: Metric) {
        tracing::debug!(message = "received metric", metric = ?item);
    }
}

impl super::direct::DirectHandler for TractHandler {
    #[tracing::instrument(name = "trace", skip_all)]
    async fn handle(&mut self, item: OneOrMany<Metric>) {
        tracing::debug!(message = "received metrics", count = item.len());
        match item {
            OneOrMany::One(item) => self.handle(item),
            OneOrMany::Many(items) => items.into_iter().for_each(|item| self.handle(item)),
        }
    }
}
