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
    pub fn build(&self, ctx: &crate::BuildContext) -> Collector {
        let refresh_kind =
            sysinfo::RefreshKind::nothing().with_memory(sysinfo::MemoryRefreshKind::everything());
        Collector {
            interval: Duration::new(self.interval, 0),
            refresh_kind,
            sender: ctx.sender.clone(),
            system: sysinfo::System::new_with_specifics(refresh_kind),
        }
    }
}

pub struct Collector {
    interval: Duration,
    sender: mpsc::Sender<OneOrMany<Metric>>,
    system: sysinfo::System,
    refresh_kind: sysinfo::RefreshKind,
}

impl crate::prelude::Worker for Collector {
    #[tracing::instrument(name = "system", skip_all)]
    async fn run(mut self) -> anyhow::Result<()> {
        let mut ticker = tokio::time::interval(self.interval);
        let hostname = sysinfo::System::host_name()
            .or_else(|| std::env::var("HOST").ok())
            .unwrap_or("unknown".into());

        let header_memory_total = chezmoi_entity::metric::MetricHeader::new("system.memory.total")
            .with_tag("host", hostname.clone());
        let header_memory_used = chezmoi_entity::metric::MetricHeader::new("system.memory.used")
            .with_tag("host", hostname.clone());
        let header_memory_ratio = chezmoi_entity::metric::MetricHeader::new("system.memory.ratio")
            .with_tag("host", hostname.clone());

        let header_swap_total = chezmoi_entity::metric::MetricHeader::new("system.swap.total")
            .with_tag("host", hostname.clone());
        let header_swap_used = chezmoi_entity::metric::MetricHeader::new("system.swap.used")
            .with_tag("host", hostname.clone());
        let header_swap_ratio = chezmoi_entity::metric::MetricHeader::new("system.swap.ratio")
            .with_tag("host", hostname.clone());

        while !self.sender.is_closed() {
            ticker.tick().await;
            self.system.refresh_specifics(self.refresh_kind);
            let timestamp = chezmoi_entity::now();

            let total_memory = self.system.total_memory() as f64;
            let used_memory = self.system.used_memory() as f64;
            let total_swap = self.system.total_swap() as f64;
            let used_swap = self.system.used_swap() as f64;

            self.sender
                .send_many(vec![
                    Metric::new(timestamp, header_memory_total.clone(), total_memory),
                    Metric::new(timestamp, header_memory_used.clone(), used_memory),
                    Metric::new(
                        timestamp,
                        header_memory_ratio.clone(),
                        used_memory * 100.0 / total_memory,
                    ),
                    Metric::new(timestamp, header_swap_total.clone(), total_swap),
                    Metric::new(timestamp, header_swap_used.clone(), used_swap),
                    Metric::new(
                        timestamp,
                        header_swap_ratio.clone(),
                        if total_swap == 0.0 {
                            0.0
                        } else {
                            used_swap * 100.0 / total_swap
                        },
                    ),
                ])
                .await;
        }
        Ok(())
    }
}
