use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc;

use crate::metric::AgentMetric;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config;

impl Config {
    pub fn build(&self) -> Exporter {
        Exporter
    }
}

#[derive(Debug, Default)]
pub struct Exporter;

impl Exporter {
    fn handle(&self, item: AgentMetric) {
        tracing::debug!(message = "received metric", metric = ?item);
    }

    pub async fn run(self, mut receiver: mpsc::Receiver<OneOrMany<AgentMetric>>) {
        while let Some(item) = receiver.recv().await {
            tracing::debug!(message = "received metrics", count = item.len());
            match item {
                OneOrMany::One(item) => self.handle(item),
                OneOrMany::Many(items) => items.into_iter().for_each(|item| self.handle(item)),
            }
        }
    }
}
