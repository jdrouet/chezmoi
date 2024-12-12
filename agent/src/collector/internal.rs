use std::time::Duration;

use chezmoi_entity::{metric::Metric, OneOrMany};
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
    pub fn from_env() -> anyhow::Result<Self> {
        let interval = crate::from_env_or("AGENT_COLLECTOR_INTERNAL_INTERVAL", default_interval)?;
        Ok(Self { interval })
    }

    pub fn build(&self, ctx: &crate::BuildContext) -> Collector {
        Collector {
            interval: Duration::new(self.interval, 0),
            sender: ctx.sender.clone(),
        }
    }
}

pub struct Collector {
    interval: Duration,
    sender: mpsc::Sender<OneOrMany<Metric>>,
}

impl crate::prelude::Worker for Collector {
    #[tracing::instrument(name = "internal", skip_all)]
    async fn run(self) -> anyhow::Result<()> {
        let mut ticker = tokio::time::interval(self.interval);
        while !self.sender.is_closed() {
            ticker.tick().await;
            self.sender
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
