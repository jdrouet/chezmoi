use std::collections::HashSet;

use chezmoi_sensor_prelude::agent::BluetoothEvent;
use tokio::task::JoinHandle;

pub mod bluetooth;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    bluetooth: bluetooth::Config,
}

impl Config {
    pub async fn build(
        &self,
        bluetooth_followed: HashSet<bluer::Address>,
    ) -> anyhow::Result<(Watcher, Receiver)> {
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
    pub bluetooth: tokio::sync::broadcast::Receiver<BluetoothEvent>,
}

impl Watcher {
    #[allow(unused, clippy::ptr_arg)]
    pub fn start(self, jobs: &mut Vec<JoinHandle<anyhow::Result<()>>>) {
        use crate::prelude::Worker;

        let Watcher { bluetooth } = self;
        jobs.push(tokio::spawn(async move { bluetooth.run().await }));
    }
}
