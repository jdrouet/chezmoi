use std::time::Duration;

use chezmoi_database::metrics::name::MetricName;
use chezmoi_database::metrics::tags::{MetricTagValue, MetricTags};
use chezmoi_database::metrics::value::MetricValue;
use chezmoi_database::metrics::Metric;
use chezmoi_helper::env::parse_env_or;
use sysinfo::{MemoryRefreshKind, RefreshKind};

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
                interval: Duration::from_secs(self.interval),
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub struct Sensor {
    inner: sysinfo::System,
    interval: Duration,
}

impl Sensor {
    async fn iterate(&mut self, buffer: &mut Vec<Metric>) -> anyhow::Result<()> {
        self.inner.refresh_all();
        let now = chezmoi_database::helper::now();
        let base_tags = MetricTags::default().maybe_with(
            "hostname",
            sysinfo::System::host_name().map(|hostname| MetricTagValue::Text(hostname.into())),
        );
        buffer.push(Metric {
            timestamp: now,
            name: MetricName::new("host.system.memory.total"),
            tags: base_tags.clone(),
            value: MetricValue::count(self.inner.total_memory()),
        });
        buffer.push(Metric {
            timestamp: now,
            name: MetricName::new("host.system.memory.used"),
            tags: base_tags.clone(),
            value: MetricValue::count(self.inner.used_memory()),
        });
        buffer.push(Metric {
            timestamp: now,
            name: MetricName::new("host.system.swap.total"),
            tags: base_tags.clone(),
            value: MetricValue::count(self.inner.total_swap()),
        });
        buffer.push(Metric {
            timestamp: now,
            name: MetricName::new("host.system.swap.used"),
            tags: base_tags,
            value: MetricValue::count(self.inner.used_swap()),
        });
        Ok(())
    }

    pub async fn run(mut self, context: super::Context) -> anyhow::Result<()> {
        let mut buffer_size: usize = 0;
        while context.state.is_running() {
            let mut buffer: Vec<Metric> = Vec::with_capacity(buffer_size);
            if let Err(error) = self.iterate(&mut buffer).await {
                tracing::error!(message = "unable to collect metrics", cause = %error);
            }
            buffer_size = buffer_size.max(buffer.len());
            if let Err(error) = context.sender.send(buffer).await {
                tracing::error!(message = "unable to send collected metrics", cause = %error);
            }
            tokio::time::sleep(self.interval).await;
        }
        Ok(())
    }
}
