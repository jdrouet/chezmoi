use chezmoi_entity::metric::Metric;
use tokio::sync::mpsc::Receiver;

use crate::collector::prelude::OneOrMany;

pub struct Trace;

impl Trace {
    fn handle(&self, item: Metric) {
        tracing::debug!(message = "received metric", metric = ?item);
    }
}

impl super::prelude::Exporter for Trace {
    async fn run(self, mut receiver: Receiver<OneOrMany<Metric>>) {
        while let Some(item) = receiver.recv().await {
            match item {
                OneOrMany::One(item) => self.handle(item),
                OneOrMany::Many(items) => items.into_iter().for_each(|item| self.handle(item)),
            }
        }
    }
}
