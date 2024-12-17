use std::collections::HashSet;

use tokio::sync::broadcast;
use tokio::task::JoinHandle;

pub mod bluetooth;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    bluetooth: bluetooth::Config,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            bluetooth: bluetooth::Config::from_env()?,
        })
    }

    pub async fn build(&self, config: &super::Config) -> anyhow::Result<(Watcher, Receiver)> {
        let mut bluetooth_followed = HashSet::new();
        config.collectors.iter().for_each(|col| match col {
            crate::collector::Config::AtcSensor(sensor) => {
                bluetooth_followed.extend(sensor.devices.iter().copied());
            }
            _ => {}
        });
        let (bluetooth, bluetooth_receiver) = self.bluetooth.build(bluetooth_followed).await?;
        Ok((
            Watcher { bluetooth },
            Receiver {
                bluetooth: bluetooth_receiver,
            },
        ))
    }
}

pub struct Watcher {
    pub bluetooth: bluetooth::Watcher,
}

pub struct Receiver {
    pub bluetooth: broadcast::Receiver<bluetooth::WatcherEvent>,
}

impl Watcher {
    pub fn start(self, jobs: &mut Vec<JoinHandle<anyhow::Result<()>>>) {
        use crate::prelude::Worker;

        let Watcher { bluetooth } = self;
        jobs.push(tokio::spawn(async move { bluetooth.run().await }));
    }
}
