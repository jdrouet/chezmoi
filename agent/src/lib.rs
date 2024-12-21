use std::path::Path;
use std::str::FromStr;

use tokio::sync::mpsc;

pub mod collector;
pub mod exporter;
pub mod prelude;
pub mod watcher;

fn from_env_or<T, F>(name: &str, default_value: F) -> anyhow::Result<T>
where
    F: FnOnce() -> T,
    T: FromStr,
    anyhow::Error: From<<T as FromStr>::Err>,
{
    if let Ok(value) = std::env::var(name) {
        Ok(T::from_str(value.as_str())?)
    } else {
        Ok(default_value())
    }
}

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
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let f = std::fs::OpenOptions::new().read(true).open(path)?;
        serde_json::from_reader(f).map_err(anyhow::Error::from)
    }

    pub async fn build(&self) -> anyhow::Result<Agent> {
        let (watcher, wreceiver) = self.watcher.build(self).await?;

        let ctx = BuildContext {
            #[cfg(feature = "watcher-bluetooth")]
            bluetooth: watcher.bluetooth.adapter.clone(),
            watcher: wreceiver,
        };

        let collectors = self.collectors.iter().map(|c| c.build(&ctx)).collect();

        Ok(Agent {
            channel_size: self.channel_size,
            watcher,
            collectors,
            exporter: self.exporter.build(),
        })
    }
}

pub struct BuildContext {
    #[cfg(feature = "watcher-bluetooth")]
    bluetooth: bluer::Adapter,
    #[allow(unused)]
    watcher: watcher::Receiver,
}

pub struct Agent {
    channel_size: usize,
    watcher: watcher::Watcher,
    collectors: Vec<collector::Collector>,
    exporter: exporter::Exporter,
}

impl Agent {
    #[tracing::instrument(name = "run", skip_all)]
    pub async fn run(self) {
        let (sender, receiver) = mpsc::channel(self.channel_size);

        let mut jobs = Vec::new();
        self.watcher.start(&mut jobs);

        jobs.extend(self.collectors.into_iter().map(|c| {
            let local_sender = sender.clone();
            tokio::spawn(async move { c.run(local_sender).await })
        }));

        self.exporter.run(receiver).await;

        while let Some(job) = jobs.pop() {
            if let Err(err) = job.await {
                tracing::error!(message = "unable to wait for job", error = %err);
            }
        }
    }
}
