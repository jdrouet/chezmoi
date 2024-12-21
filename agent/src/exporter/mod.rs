use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;
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
    pub fn from_env() -> anyhow::Result<Self> {
        match std::env::var("AGENT_EXPORTER_TYPE").as_deref() {
            Ok("http") => Ok(Self::Http(http::Config::from_env()?)),
            _ => Ok(Self::Trace(trace::Config {})),
        }
    }

    pub fn build(&self, receiver: Receiver<OneOrMany<Metric>>) -> Exporter {
        match self {
            Self::Http(inner) => Exporter::Http(inner.build(receiver)),
            Self::Trace(inner) => Exporter::Trace(inner.build(receiver)),
        }
    }
}

pub enum Exporter {
    Http(http::Exporter),
    Trace(trace::Exporter),
}

impl Exporter {
    pub async fn run(self) {
        match self {
            Self::Http(inner) => inner.run().await,
            Self::Trace(inner) => inner.run().await,
        }
    }
}
