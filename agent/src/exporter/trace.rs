use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc;

#[derive(Debug, serde::Deserialize)]
pub struct Config {}

impl Config {
    pub fn build(&self, receiver: mpsc::Receiver<OneOrMany<Metric>>) -> Exporter {
        Exporter { receiver }
    }
}

pub struct Exporter {
    receiver: mpsc::Receiver<OneOrMany<Metric>>,
}

impl Exporter {
    fn handle(&self, item: Metric) {
        tracing::debug!(message = "received metric", metric = ?item);
    }

    pub async fn run(mut self) {
        while let Some(item) = self.receiver.recv().await {
            tracing::debug!(message = "received metrics", count = item.len());
            match item {
                OneOrMany::One(item) => self.handle(item),
                OneOrMany::Many(items) => items.into_iter().for_each(|item| self.handle(item)),
            }
        }
    }
}
