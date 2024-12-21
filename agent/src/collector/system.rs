use std::time::Duration;

use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc;

use crate::metric::AgentMetric;

use super::helper::{CachedSender, Hostname};

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
        let refresh_kind =
            sysinfo::RefreshKind::nothing().with_memory(sysinfo::MemoryRefreshKind::everything());
        Collector {
            interval: Duration::new(self.interval, 0),
            refresh_kind,
            system: sysinfo::System::new_with_specifics(refresh_kind),
        }
    }
}

#[derive(Clone, Debug, Hash)]
pub enum AgentMetricName {
    MemoryTotal,
    MemoryUsed,
    MemoryRatio,
    SwapTotal,
    SwapUsed,
    SwapRatio,
}

impl AgentMetricName {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::MemoryTotal => "system.memory.total",
            Self::MemoryUsed => "system.memory.used",
            Self::MemoryRatio => "system.memory.ratio",
            Self::SwapTotal => "system.swap.total",
            Self::SwapUsed => "system.swap.used",
            Self::SwapRatio => "system.swap.ratio",
        }
    }
}

impl serde::Serialize for AgentMetricName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Clone, Debug, Hash, serde::Serialize)]
pub struct AgentMetricTags {
    hostname: Hostname,
}

#[derive(Clone, Debug, Hash, serde::Serialize)]
pub struct AgentMetricHeader {
    name: AgentMetricName,
    tags: AgentMetricTags,
}

impl AgentMetricHeader {
    pub const fn new(name: AgentMetricName, hostname: Hostname) -> Self {
        Self {
            name,
            tags: AgentMetricTags { hostname },
        }
    }
}

pub struct Collector {
    interval: Duration,
    system: sysinfo::System,
    refresh_kind: sysinfo::RefreshKind,
}

impl Collector {
    #[tracing::instrument(name = "system", skip_all)]
    pub async fn run(mut self, sender: mpsc::Sender<OneOrMany<AgentMetric>>) -> anyhow::Result<()> {
        let mut ticker = tokio::time::interval(self.interval);
        let mut sender = CachedSender::new(6, self.interval.as_secs(), sender);

        let hostname = Hostname::default();

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
                    AgentMetric::new(
                        timestamp,
                        AgentMetricHeader::new(AgentMetricName::MemoryTotal, hostname.clone()),
                        total_memory,
                    ),
                    AgentMetric::new(
                        timestamp,
                        AgentMetricHeader::new(AgentMetricName::MemoryUsed, hostname.clone()),
                        used_memory,
                    ),
                    AgentMetric::new(
                        timestamp,
                        AgentMetricHeader::new(AgentMetricName::MemoryRatio, hostname.clone()),
                        used_memory * 100.0 / total_memory,
                    ),
                    AgentMetric::new(
                        timestamp,
                        AgentMetricHeader::new(AgentMetricName::SwapTotal, hostname.clone()),
                        total_swap,
                    ),
                    AgentMetric::new(
                        timestamp,
                        AgentMetricHeader::new(AgentMetricName::SwapUsed, hostname.clone()),
                        used_swap,
                    ),
                    AgentMetric::new(
                        timestamp,
                        AgentMetricHeader::new(AgentMetricName::SwapRatio, hostname.clone()),
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
