use std::time::Duration;

use chezmoi_entity::metric::Metric;

use crate::collector::prelude::Context;

#[derive(Debug)]
pub struct Config {
    interval: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self { interval: 10 }
    }
}

impl Config {
    pub fn build(&self) -> Collector {
        Collector {
            interval: Duration::new(self.interval, 0),
        }
    }
}

pub struct Collector {
    interval: Duration,
}

impl super::prelude::Collector for Collector {
    #[tracing::instrument(name = "internal", skip_all)]
    async fn run(&mut self, ctx: Context) -> anyhow::Result<()> {
        let mut ticker = tokio::time::interval(self.interval);
        while !ctx.is_closing() {
            ticker.tick().await;
            ctx.send(Metric {
                timestamp: chezmoi_entity::now(),
                header: chezmoi_entity::metric::MetricHeader::new("internal.queue.size"),
                value: ctx.queue_size() as f64,
            })
            .await;
        }
        Ok(())
    }
}
