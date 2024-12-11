use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;

#[derive(Debug, Default)]
pub struct TraceHandler;

impl TraceHandler {
    fn handle(&mut self, item: Metric) {
        tracing::debug!(message = "received metric", metric = ?item);
    }
}

impl super::prelude::Handler for TraceHandler {
    #[tracing::instrument(name = "trace", skip_all)]
    async fn handle(&mut self, item: OneOrMany<Metric>) {
        tracing::debug!(message = "received metrics", count = item.len());
        match item {
            OneOrMany::One(item) => self.handle(item),
            OneOrMany::Many(items) => items.into_iter().for_each(|item| self.handle(item)),
        }
    }
}
