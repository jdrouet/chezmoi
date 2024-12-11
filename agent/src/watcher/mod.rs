use tokio::{sync::broadcast, task::JoinHandle};

pub mod bluetooth;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    bluetooth: bluetooth::Config,
}

impl Config {
    pub async fn build(&self, _config: &super::Config) -> anyhow::Result<(Watcher, Receiver)> {
        let (bluetooth, bluetooth_receiver) = self.bluetooth.build(std::iter::empty()).await?;
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
