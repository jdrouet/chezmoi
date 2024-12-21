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
        Collector {
            interval: Duration::new(self.interval, 0),
        }
    }
}

#[derive(Clone, Debug, Hash)]
pub enum AgentMetricName {
    QueueSize,
}

impl AgentMetricName {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::QueueSize => "internal.queue.size",
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
}

impl Collector {
    #[tracing::instrument(name = "internal", skip_all)]
    pub async fn run(self, sender: mpsc::Sender<OneOrMany<AgentMetric>>) -> anyhow::Result<()> {
        let mut ticker = tokio::time::interval(self.interval);
        let mut sender = CachedSender::new(1, self.interval.as_secs(), sender);

        let hostname = Hostname::default();

        while !sender.is_closed() {
            ticker.tick().await;
            sender
                .send_one(AgentMetric::new(
                    chezmoi_entity::now(),
                    AgentMetricHeader::new(AgentMetricName::QueueSize, hostname.clone()),
                    0.0,
                ))
                .await;
        }
        Ok(())
    }
}
