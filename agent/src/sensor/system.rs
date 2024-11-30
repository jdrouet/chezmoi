use std::time::Duration;

use chezmoi_database::metrics::entity::{Metric, MetricValue};
use chezmoi_database::metrics::MetricHeader;
use chezmoi_helper::env::parse_env_or;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};
use tokio::time::Interval;

use super::Collector;

pub const GLOBAL_CPU_USAGE: &str = "host.system.global_cpu.usage";
pub const MEMORY_TOTAL: &str = "host.system.memory.total";
pub const MEMORY_USED: &str = "host.system.memory.used";
pub const SWAP_TOTAL: &str = "host.system.swap.total";
pub const SWAP_USED: &str = "host.system.swap.used";

pub(crate) struct Config {
    enabled: bool,
    interval: u64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            enabled: parse_env_or("SENSOR_SYSTEM_ENABLED", false)?,
            interval: parse_env_or("SENSOR_SYSTEM_INTERVAL", 10)?,
        })
    }

    pub fn build(self) -> anyhow::Result<Option<Sensor>> {
        if self.enabled {
            Ok(Some(Sensor {
                inner: sysinfo::System::new_with_specifics(
                    RefreshKind::new()
                        .with_cpu(CpuRefreshKind::everything())
                        .with_memory(MemoryRefreshKind::everything()),
                ),
                interval: tokio::time::interval(Duration::from_secs(self.interval)),
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub(crate) struct Sensor {
    inner: sysinfo::System,
    interval: Interval,
}

impl Sensor {
    async fn iterate(&mut self, buffer: &mut Collector) -> anyhow::Result<()> {
        self.inner.refresh_all();
        let now = chezmoi_database::helper::now();
        buffer.collect(Metric {
            timestamp: now,
            header: MetricHeader::new(GLOBAL_CPU_USAGE),
            value: MetricValue::gauge(self.inner.global_cpu_usage() as f64),
        });
        buffer.collect(Metric {
            timestamp: now,
            header: MetricHeader::new(MEMORY_TOTAL),
            value: MetricValue::gauge(self.inner.total_memory() as f64),
        });
        buffer.collect(Metric {
            timestamp: now,
            header: MetricHeader::new(MEMORY_USED),
            value: MetricValue::gauge(self.inner.used_memory() as f64),
        });
        buffer.collect(Metric {
            timestamp: now,
            header: MetricHeader::new(SWAP_TOTAL),
            value: MetricValue::gauge(self.inner.total_swap() as f64),
        });
        buffer.collect(Metric {
            timestamp: now,
            header: MetricHeader::new(SWAP_USED),
            value: MetricValue::gauge(self.inner.used_swap() as f64),
        });
        Ok(())
    }

    pub async fn run(mut self, context: super::Context) -> anyhow::Result<()> {
        let mut collector = super::Collector::new(super::Cache::default(), 5);
        while context.state.is_running() {
            self.interval.tick().await;
            if let Err(error) = self.iterate(&mut collector).await {
                tracing::error!(message = "unable to collect metrics", cause = %error);
            }

            if let Some(events) = collector.flush() {
                if let Err(error) = context.sender.send(events).await {
                    tracing::error!(message = "unable to send collected metrics", cause = %error);
                }
            }
        }
        Ok(())
    }
}
