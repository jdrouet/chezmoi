use std::time::Duration;

use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc;

use super::prelude::SenderExt;

pub const fn default_interval() -> u64 {
    10
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_interval")]
    interval: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self { interval: 10 }
    }
}

impl Config {
    pub fn build(&self, _ctx: &crate::BuildContext) -> Collector {
        Collector {
            interval: Duration::new(self.interval, 0),
        }
    }
}

pub struct Collector {
    interval: Duration,
}

impl Collector {
    #[tracing::instrument(name = "internal", skip_all)]
    pub async fn run(self, sender: mpsc::Sender<OneOrMany<Metric>>) -> anyhow::Result<()> {
        let mut ticker = tokio::time::interval(self.interval);
        while !sender.is_closed() {
            ticker.tick().await;
            sender
                .send_one(Metric {
                    timestamp: chezmoi_entity::now(),
                    header: chezmoi_entity::metric::MetricHeader::new("internal.queue.size"),
                    value: 0.0,
                })
                .await;
        }
        Ok(())
    }
}
