use std::sync::Arc;
use std::time::Duration;

use chezmoi_entity::metric::MetricHeader;
use chezmoi_sensor_prelude::agent::cache::CachedSender;
use chezmoi_sensor_prelude::agent::prelude::SensorSender;
use chezmoi_sensor_prelude::agent::{AgentMetric, BuildContext};

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

impl chezmoi_sensor_prelude::agent::prelude::Config for Config {
    type Output = Sensor;

    async fn build(&self, ctx: &BuildContext) -> anyhow::Result<Sensor> {
        let headers = Headers::new(ctx.hostname.as_str());
        let refresh_kind =
            sysinfo::RefreshKind::nothing().with_memory(sysinfo::MemoryRefreshKind::everything());
        Ok(Sensor {
            headers,
            interval: Duration::new(self.interval, 0),
            refresh_kind,
            system: sysinfo::System::new_with_specifics(refresh_kind),
        })
    }
}

pub struct Headers {
    memory_total: Arc<MetricHeader<'static>>,
    memory_used: Arc<MetricHeader<'static>>,
    memory_ratio: Arc<MetricHeader<'static>>,
    swap_total: Arc<MetricHeader<'static>>,
    swap_used: Arc<MetricHeader<'static>>,
    swap_ratio: Arc<MetricHeader<'static>>,
}

impl Headers {
    pub fn new(hostname: &str) -> Self {
        Self {
            memory_total: Arc::new(
                MetricHeader::new(crate::MEMORY_TOTAL).with_tag("hostname", hostname.to_string()),
            ),
            memory_used: Arc::new(
                MetricHeader::new(crate::MEMORY_USED).with_tag("hostname", hostname.to_string()),
            ),
            memory_ratio: Arc::new(
                MetricHeader::new(crate::MEMORY_RATIO).with_tag("hostname", hostname.to_string()),
            ),
            swap_total: Arc::new(
                MetricHeader::new(crate::SWAP_TOTAL).with_tag("hostname", hostname.to_string()),
            ),
            swap_used: Arc::new(
                MetricHeader::new(crate::SWAP_USED).with_tag("hostname", hostname.to_string()),
            ),
            swap_ratio: Arc::new(
                MetricHeader::new(crate::SWAP_RATIO).with_tag("hostname", hostname.to_string()),
            ),
        }
    }
}

pub struct Sensor {
    headers: Headers,
    interval: Duration,
    system: sysinfo::System,
    refresh_kind: sysinfo::RefreshKind,
}

impl chezmoi_sensor_prelude::agent::prelude::Sensor for Sensor {
    #[tracing::instrument(name = "system", skip_all)]
    async fn run(mut self, sender: SensorSender) -> anyhow::Result<()> {
        let mut ticker = tokio::time::interval(self.interval);
        let mut sender = CachedSender::new(6, self.interval.as_secs(), sender);

        while !sender.is_closed() {
            ticker.tick().await;
            self.system.refresh_specifics(self.refresh_kind);
            let timestamp = chezmoi_entity::now();

            let total_memory = self.system.total_memory() as f64;
            let used_memory = self.system.used_memory() as f64;
            let total_swap = self.system.total_swap() as f64;
            let used_swap = self.system.used_swap() as f64;

            sender
                .send_many([
                    AgentMetric::new(timestamp, self.headers.memory_total.clone(), total_memory),
                    AgentMetric::new(timestamp, self.headers.memory_used.clone(), used_memory),
                    AgentMetric::new(
                        timestamp,
                        self.headers.memory_ratio.clone(),
                        used_memory * 100.0 / total_memory,
                    ),
                    AgentMetric::new(timestamp, self.headers.swap_total.clone(), total_swap),
                    AgentMetric::new(timestamp, self.headers.swap_used.clone(), used_swap),
                    AgentMetric::new(
                        timestamp,
                        self.headers.swap_ratio.clone(),
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
