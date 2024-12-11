use std::path::Path;

use chezmoi_entity::{metric::Metric, OneOrMany};
use tokio::sync::mpsc;

pub mod collector;
pub mod exporter;
pub mod prelude;
pub mod watcher;

const fn default_channel_size() -> usize {
    200
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_channel_size")]
    channel_size: usize,
    #[serde(default)]
    watcher: watcher::Config,
    #[serde(default)]
    collectors: Vec<collector::Config>,
    exporter: exporter::Config,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let f = std::fs::OpenOptions::new().read(true).open(path)?;
        serde_json::from_reader(f).map_err(std::io::Error::other)
    }

    pub async fn build(&self) -> anyhow::Result<Agent> {
        let (watcher, wreceiver) = self.watcher.build(self).await?;
        let (sender, receiver) = mpsc::channel(self.channel_size);

        let ctx = BuildContext {
            sender,
            watcher: wreceiver,
        };

        let collectors = self.collectors.iter().map(|c| c.build(&ctx)).collect();

        Ok(Agent {
            watcher,
            collectors,
            exporter: self.exporter.build(receiver),
        })
    }
}

pub struct BuildContext {
    sender: mpsc::Sender<OneOrMany<Metric>>,
    #[allow(unused)]
    watcher: watcher::Receiver,
}

pub struct Agent {
    watcher: watcher::Watcher,
    collectors: Vec<collector::Collector>,
    exporter: exporter::Exporter,
}

impl Agent {
    #[tracing::instrument(name = "run", skip_all)]
    pub async fn run(self) {
        use crate::exporter::prelude::Exporter;
        use crate::prelude::Worker;

        let mut jobs = Vec::new();
        self.watcher.start(&mut jobs);

        jobs.extend(
            self.collectors
                .into_iter()
                .map(|c| tokio::spawn(async move { c.run().await })),
        );

        self.exporter.run().await;

        while let Some(job) = jobs.pop() {
            if let Err(err) = job.await {
                tracing::error!(message = "unable to wait for job", error = %err);
            }
        }
    }
}
