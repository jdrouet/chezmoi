use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc::Receiver;

pub mod batch;
pub mod cache;
pub mod direct;
pub mod http;
pub mod queue;
pub mod trace;

pub mod prelude;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Config {
    Http {
        address: String,
        #[serde(default = "crate::exporter::batch::default_capacity")]
        batch_capacity: usize,
        #[serde(default = "crate::exporter::batch::default_interval")]
        batch_interval: u64,
        #[serde(default = "crate::exporter::cache::default_size")]
        cache_size: usize,
        #[serde(default = "crate::exporter::cache::default_ttl")]
        cache_ttl: u64,
    },
    Trace,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        match std::env::var("AGENT_EXPORTER_TYPE").as_deref() {
            Ok("http") => Ok(Self::Http {
                address: std::env::var("AGENT_EXPORTER_ADDRESS")
                    .unwrap_or_else(|_| String::from("http://localhost:3000/api/metrics")),
                batch_capacity: crate::from_env_or(
                    "AGENT_EXPORTER_BATCH_CAPACITY",
                    crate::exporter::batch::default_capacity,
                )?,
                batch_interval: crate::from_env_or(
                    "AGENT_EXPORTER_BATCH_INTERVAL",
                    crate::exporter::batch::default_interval,
                )?,
                cache_size: crate::from_env_or(
                    "AGENT_EXPORTER_CACHE_SIZE",
                    crate::exporter::cache::default_size,
                )?,
                cache_ttl: crate::from_env_or(
                    "AGENT_EXPORTER_CACHE_TTL",
                    crate::exporter::cache::default_ttl,
                )?,
            }),
            _ => Ok(Self::Trace),
        }
    }

    pub fn build(&self, receiver: Receiver<OneOrMany<Metric>>) -> Exporter {
        match self {
            Self::Http {
                address,
                batch_capacity,
                batch_interval,
                cache_size,
                cache_ttl,
            } => Exporter::Http(batch::BatchExporter::new(
                receiver,
                *batch_capacity,
                *batch_interval,
                cache::CacheLayer::new(
                    *cache_size,
                    *cache_ttl,
                    http::HttpHandler::new(address.clone()),
                ),
            )),
            Self::Trace => {
                Exporter::Trace(direct::DirectExporter::new(receiver, trace::TraceHandler))
            }
        }
    }
}

pub enum Exporter {
    Http(batch::BatchExporter<cache::CacheLayer<http::HttpHandler>>),
    Trace(direct::DirectExporter<trace::TraceHandler>),
}

impl prelude::Exporter for Exporter {
    async fn run(self) {
        match self {
            Self::Http(inner) => inner.run().await,
            Self::Trace(inner) => inner.run().await,
        }
    }
}
