use std::time::Duration;

use chezmoi_database::metrics::entity::{Metric, MetricValue};
use chezmoi_database::metrics::{MetricHeader, MetricName, MetricTagValue, MetricTags};
use chezmoi_helper::env::parse_env_or;
use sysinfo::{MemoryRefreshKind, RefreshKind};
use tokio::time::Interval;

use super::Collector;

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
                    RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
                ),
                interval: tokio::time::interval(Duration::from_secs(self.interval)),
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub struct Sensor {
    inner: sysinfo::System,
    interval: Interval,
}

impl Sensor {
    async fn iterate(&mut self, buffer: &mut Collector) -> anyhow::Result<()> {
        self.inner.refresh_all();
        let now = chezmoi_database::helper::now();
        let hostname = super::Hostname::default();
        let base_tags = MetricTags::default()
            .maybe_with("hostname", hostname.inner().map(MetricTagValue::ArcText));
        buffer.collect(Metric {
            timestamp: now,
            header: MetricHeader {
                name: MetricName::new("host.system.memory.total"),
                tags: base_tags.clone(),
            },
            value: MetricValue::gauge(self.inner.total_memory() as f64),
        });
        buffer.collect(Metric {
            timestamp: now,
            header: MetricHeader {
                name: MetricName::new("host.system.memory.used"),
                tags: base_tags.clone(),
            },
            value: MetricValue::gauge(self.inner.used_memory() as f64),
        });
        buffer.collect(Metric {
            timestamp: now,
            header: MetricHeader {
                name: MetricName::new("host.system.swap.total"),
                tags: base_tags.clone(),
            },
            value: MetricValue::gauge(self.inner.total_swap() as f64),
        });
        buffer.collect(Metric {
            timestamp: now,
            header: MetricHeader {
                name: MetricName::new("host.system.swap.used"),
                tags: base_tags,
            },
            value: MetricValue::gauge(self.inner.used_swap() as f64),
        });
        Ok(())
    }

    pub async fn run(mut self, context: super::Context) -> anyhow::Result<()> {
        let mut collector = super::Collector::new(super::Cache::default(), 4);
        while context.state.is_running() {
            self.interval.tick().await;
            if let Err(error) = self.iterate(&mut collector).await {
                tracing::error!(message = "unable to collect metrics", cause = %error);
            }
            let buffer = collector.flush();
            if let Err(error) = context.sender.send(buffer).await {
                tracing::error!(message = "unable to send collected metrics", cause = %error);
            }
        }
        Ok(())
    }
}
