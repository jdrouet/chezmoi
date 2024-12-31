use chezmoi_entity::OneOrMany;
use chezmoi_sensor_prelude::agent::AgentMetric;
use tokio::sync::mpsc::Receiver;

pub mod http;
pub mod trace;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Config {
    Http(http::Config),
    Trace(trace::Config),
}

impl Config {
    pub fn build(&self) -> Exporter {
        match self {
            Self::Http(inner) => Exporter::Http(inner.build()),
            Self::Trace(inner) => Exporter::Trace(inner.build()),
        }
    }
}

pub enum Exporter {
    Http(http::Exporter),
    Trace(trace::Exporter),
}

impl Exporter {
    pub async fn run(self, receiver: Receiver<OneOrMany<AgentMetric>>) {
        match self {
            Self::Http(inner) => inner.run(receiver).await,
            Self::Trace(inner) => inner.run(receiver).await,
        }
    }
}
