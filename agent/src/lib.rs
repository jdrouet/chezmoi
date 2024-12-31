use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;

use chezmoi_sensor_prelude::agent::{BluetoothBuildContext, BuildContext};
use tokio::sync::mpsc;

// pub mod collector;
pub mod exporter;
// mod metric;
pub mod prelude;
pub mod sensor;
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
    collectors: Vec<sensor::Config>,
    exporter: exporter::Config,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let f = std::fs::OpenOptions::new().read(true).open(path)?;
        serde_json::from_reader(f).map_err(anyhow::Error::from)
    }

    fn bluetooth_addresses(&self) -> HashSet<bluer::Address> {
        let mut set = HashSet::new();
        self.collectors
            .iter()
            .for_each(|c| c.bluetooth_addresses(&mut set));
        set
    }

    pub async fn build(&self) -> anyhow::Result<Agent> {
        use chezmoi_sensor_prelude::agent::prelude::Config;

        let (watcher, wreceiver) = self.watcher.build(self.bluetooth_addresses()).await?;

        let ctx = BuildContext {
            bluetooth: BluetoothBuildContext {
                adapter: watcher.bluetooth.adapter.clone(),
                receiver: wreceiver.bluetooth,
            },
            hostname: std::env::var("HOSTNAME").unwrap_or_else(|_| String::from("unknown")),
        };

        let mut collectors = Vec::with_capacity(self.collectors.len());
        for c in self.collectors.iter() {
            collectors.push(c.build(&ctx).await?);
        }

        Ok(Agent {
            channel_size: self.channel_size,
            watcher,
            collectors,
            exporter: self.exporter.build(),
        })
    }
}

pub struct Agent {
    channel_size: usize,
    watcher: watcher::Watcher,
    collectors: Vec<sensor::Sensor>,
    exporter: exporter::Exporter,
}

impl Agent {
    #[tracing::instrument(name = "run", skip_all)]
    pub async fn run(self) {
        use chezmoi_sensor_prelude::agent::prelude::Sensor;

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
