use std::path::Path;

use collector::prelude::Collector;
use exporter::prelude::Exporter;
use tokio::sync::mpsc;

pub mod collector;
pub mod exporter;

const fn default_channel_size() -> usize {
    200
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_channel_size")]
    channel_size: usize,
    #[serde(default)]
    collectors: Vec<collector::Config>,
    exporter: exporter::Config,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let f = std::fs::OpenOptions::new().read(true).open(path)?;
        serde_json::from_reader(f).map_err(std::io::Error::other)
    }

    pub fn build(&self) -> Agent {
        Agent {
            channel_size: self.channel_size,
            collectors: self.collectors.iter().map(|c| c.build()).collect(),
            exporter: self.exporter.build(),
        }
    }
}

pub struct Agent {
    channel_size: usize,
    collectors: Vec<collector::Collector>,
    exporter: exporter::Exporter,
}

impl Agent {
    #[tracing::instrument(name = "run", skip_all)]
    pub async fn run(self) {
        let (sender, receiver) = mpsc::channel(self.channel_size);

        let ctx = collector::prelude::Context::new(sender);
        let mut jobs = self
            .collectors
            .into_iter()
            .map(|c| {
                let local_ctx = ctx.clone();
                tokio::spawn(async move { c.run(local_ctx).await })
            })
            .collect::<Vec<_>>();

        self.exporter.run(receiver).await;

        while let Some(job) = jobs.pop() {
            if let Err(err) = job.await {
                tracing::error!(message = "unable to wait for job", error = %err);
            }
        }
    }
}
