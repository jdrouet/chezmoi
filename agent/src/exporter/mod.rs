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
