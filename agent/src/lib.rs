#[cfg(feature = "bluetooth")]
use std::collections::HashSet;
#[cfg(feature = "bluetooth")]
use std::str::FromStr;

use chezmoi_database::metrics::entity::Metric;
#[cfg(feature = "bluetooth")]
use tokio::sync::broadcast;
use tokio::sync::mpsc;

pub mod sensor;
pub mod watcher;

pub const HOSTNAME: &str = "hostname";
pub const ADDRESS: &str = "address";

#[cfg(feature = "bluetooth")]
async fn default_bt_adapter() -> anyhow::Result<bluer::Adapter> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    Ok(adapter)
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    #[cfg(feature = "sensor-atc-thermometer")]
    #[serde(default)]
    atc_thermometer: sensor::ConfigWrapper<sensor::atc_thermometer::Config>,
    #[cfg(feature = "sensor-bt-scanner")]
    #[serde(default)]
    bt_scanner: sensor::ConfigWrapper<sensor::bt_scanner::Config>,
    #[cfg(feature = "sensor-miflora")]
    #[serde(default)]
    miflora: sensor::ConfigWrapper<sensor::miflora::Config>,
    #[serde(default)]
    system: sensor::ConfigWrapper<sensor::system::Config>,
}

impl Config {
    #[cfg(feature = "bluetooth")]
    fn bt_addresses(&self) -> HashSet<bluer::Address> {
        let mut res = HashSet::default();
        #[cfg(feature = "sensor-atc-thermometer")]
        if self.atc_thermometer.enabled {
            res.extend(
                self.atc_thermometer
                    .inner
                    .devices
                    .iter()
                    .filter_map(|addr| bluer::Address::from_str(addr.as_str()).ok()),
            );
        }
        #[cfg(feature = "sensor-miflora")]
        if self.miflora.enabled {
            res.extend(
                self.miflora
                    .inner
                    .devices
                    .iter()
                    .filter_map(|addr| bluer::Address::from_str(addr.as_str()).ok()),
            );
        }
        res
    }

    #[cfg(feature = "bluetooth")]
    fn bt_watcher(&self, adapter: bluer::Adapter) -> Option<watcher::bluetooth::Watcher> {
        if self.atc_thermometer.enabled || self.miflora.enabled {
            Some(watcher::bluetooth::Watcher::new(
                adapter,
                self.bt_addresses(),
            ))
        } else {
            None
        }
    }

    #[cfg(feature = "sensor-atc-thermometer")]
    fn atc_thermometer(&self, adapter: bluer::Adapter) -> Option<sensor::atc_thermometer::Sensor> {
        if self.atc_thermometer.enabled {
            Some(self.atc_thermometer.inner.build(adapter))
        } else {
            None
        }
    }

    #[cfg(feature = "sensor-bt-scanner")]
    fn bt_scanner(&self, adapter: bluer::Adapter) -> Option<sensor::bt_scanner::Sensor> {
        if self.bt_scanner.enabled {
            Some(self.bt_scanner.inner.build(adapter))
        } else {
            None
        }
    }

    #[cfg(feature = "sensor-miflora")]
    fn miflora(&self, adapter: bluer::Adapter) -> Option<sensor::miflora::Sensor> {
        if self.miflora.enabled {
            Some(self.miflora.inner.build(adapter))
        } else {
            None
        }
    }

    fn system(&self) -> Option<sensor::system::Sensor> {
        if self.system.enabled {
            Some(self.system.inner.build())
        } else {
            None
        }
    }

    pub async fn build(self) -> anyhow::Result<Agent> {
        #[cfg(feature = "bluetooth")]
        let bt_adapter = default_bt_adapter().await?;

        Ok(Agent {
            #[cfg(feature = "bluetooth")]
            bt_watcher: self.bt_watcher(bt_adapter.clone()),

            #[cfg(feature = "sensor-atc-thermometer")]
            atc_thermometer: self.atc_thermometer(bt_adapter.clone()),
            #[cfg(feature = "sensor-bt-scanner")]
            bt_scanner: self.bt_scanner(bt_adapter.clone()),
            #[cfg(feature = "sensor-miflora")]
            miflora: self.miflora(bt_adapter.clone()),
            system: self.system(),
        })
    }
}

#[tracing::instrument(name = "collector", skip_all)]
async fn collect(
    database: chezmoi_database::Client,
    mut receiver: mpsc::Receiver<Vec<Metric>>,
) -> anyhow::Result<()> {
    while let Some(batch) = receiver.recv().await {
        if batch.is_empty() {
            continue;
        }
        match chezmoi_database::metrics::entity::create::Command::new(&batch)
            .execute(database.as_ref())
            .await
        {
            Ok(count) => tracing::debug!(message = "stored events", count = count),
            Err(error) => {
                tracing::error!(message = "unable to store received metrics", cause = %error)
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Agent {
    #[cfg(feature = "bluetooth")]
    bt_watcher: Option<watcher::bluetooth::Watcher>,
    #[cfg(feature = "sensor-atc-thermometer")]
    atc_thermometer: Option<sensor::atc_thermometer::Sensor>,
    #[cfg(feature = "sensor-bt-scanner")]
    bt_scanner: Option<sensor::bt_scanner::Sensor>,
    #[cfg(feature = "sensor-miflora")]
    miflora: Option<sensor::miflora::Sensor>,
    system: Option<sensor::system::Sensor>,
}

impl Agent {
    pub async fn run(self, database: chezmoi_database::Client) -> anyhow::Result<()> {
        let (sender, receiver) = mpsc::channel::<Vec<Metric>>(100);
        #[cfg(feature = "bluetooth")]
        let (bt_sender, bt_receiver) = broadcast::channel::<watcher::bluetooth::WatcherEvent>(100);

        let context = sensor::Context::new(true, sender);

        let mut tasks = Vec::new();
        #[cfg(feature = "sensor-atc-thermometer")]
        if let Some(sensor) = self.atc_thermometer {
            let ctx = context.clone();
            let rcv = bt_receiver.resubscribe();
            tasks.push(tokio::spawn(async move { sensor.run(ctx, rcv).await }));
        }
        #[cfg(feature = "sensor-bt-scanner")]
        if let Some(sensor) = self.bt_scanner {
            let ctx = context.clone();
            let rcv = bt_receiver.resubscribe();
            tasks.push(tokio::spawn(async move { sensor.run(ctx, rcv).await }));
        }
        #[cfg(feature = "sensor-miflora")]
        if let Some(sensor) = self.miflora {
            let ctx = context.clone();
            let rcv = bt_receiver.resubscribe();
            tasks.push(tokio::spawn(async move { sensor.run(ctx, rcv).await }));
        }
        if let Some(sensor) = self.system {
            let ctx = context.clone();
            tasks.push(tokio::spawn(async move { sensor.run(ctx).await }));
        }

        #[cfg(feature = "bluetooth")]
        if let Some(watcher) = self.bt_watcher {
            let ctx = context.clone();
            tasks.push(tokio::spawn(
                async move { watcher.run(ctx, bt_sender).await },
            ));
        }

        collect(database, receiver).await?;

        while let Some(sensor) = tasks.pop() {
            match sensor.await {
                Ok(Ok(_)) => {}
                Ok(Err(inner)) => tracing::error!(message = "task failed", cause = %inner),
                Err(inner) => tracing::error!(message = "unable to join taask", cause = %inner),
            }
        }

        Ok(())
    }
}
