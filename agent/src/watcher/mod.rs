use tokio::task::JoinHandle;

#[cfg(feature = "watcher-bluetooth")]
pub mod bluetooth;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    #[cfg(feature = "watcher-bluetooth")]
    #[serde(default)]
    bluetooth: bluetooth::Config,
}

impl Config {
    #[cfg(feature = "watcher-bluetooth")]
    pub async fn build(&self, config: &super::Config) -> anyhow::Result<(Watcher, Receiver)> {
        #[allow(unused)]
        let mut bluetooth_followed = std::collections::HashSet::new();
        config.collectors.iter().for_each(|col| match col {
            #[cfg(feature = "collector-atc-sensor")]
            crate::collector::Config::AtcSensor(sensor) => {
                bluetooth_followed.extend(
                    sensor
                        .devices
                        .iter()
                        .map(|addr| bluer::Address::new(addr.0)),
                );
            }
            #[cfg(feature = "collector-miflora-sensor")]
            crate::collector::Config::MifloraSensor(sensor) => {
                bluetooth_followed.extend(
                    sensor
                        .devices
                        .iter()
                        .map(|addr| bluer::Address::new(addr.0)),
                );
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

    #[cfg(not(feature = "watcher-bluetooth"))]
    pub async fn build(&self, _config: &super::Config) -> anyhow::Result<(Watcher, Receiver)> {
        Ok((Watcher {}, Receiver {}))
    }
}

pub struct Watcher {
    #[cfg(feature = "watcher-bluetooth")]
    pub bluetooth: bluetooth::Watcher,
}

pub struct Receiver {
    #[cfg(feature = "watcher-bluetooth")]
    pub bluetooth: tokio::sync::broadcast::Receiver<bluetooth::WatcherEvent>,
}

impl Watcher {
    #[allow(unused, clippy::ptr_arg)]
    pub fn start(self, jobs: &mut Vec<JoinHandle<anyhow::Result<()>>>) {
        use crate::prelude::Worker;

        let Watcher {
            #[cfg(feature = "watcher-bluetooth")]
            bluetooth,
        } = self;
        #[cfg(feature = "watcher-bluetooth")]
        jobs.push(tokio::spawn(async move { bluetooth.run().await }));
    }
}
