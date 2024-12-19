use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::Duration;

use chezmoi_entity::address::Address;
use chezmoi_entity::metric::{Metric, MetricHeader};
use chezmoi_entity::{now, OneOrMany};
use tokio::sync::{broadcast, mpsc};

use crate::collector::prelude::SenderExt;
use crate::watcher::bluetooth::WatcherEvent;

pub const DEVICE_BATTERY: &str = "miflora.battery";
pub const DEVICE_TEMPERATURE: &str = "miflora.temperature";
pub const DEVICE_BRIGHTNESS: &str = "miflora.brightness";
pub const DEVICE_CONDUCTIVITY: &str = "miflora.conductivity";
pub const DEVICE_MOISTURE: &str = "miflora.moisture";

/// default interval between historical fetch
///
/// defaults to 24h
pub const fn default_interval() -> u64 {
    60 * 60 * 24
}

#[derive(Clone, Debug)]
pub struct PollingModeParsingError(pub String);

impl std::fmt::Display for PollingModeParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unknown polling mode provided {:?}, expected \"history\" or \"realtime\"",
            self.0
        )
    }
}

impl std::error::Error for PollingModeParsingError {}

#[derive(Clone, Copy, Debug, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PollingMode {
    #[default]
    History,
    Realtime,
}

impl FromStr for PollingMode {
    type Err = PollingModeParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "history" => Ok(Self::History),
            "realtime" => Ok(Self::Realtime),
            other => Err(PollingModeParsingError(other.to_string())),
        }
    }
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_interval")]
    pub interval: u64,
    #[serde(default)]
    pub mode: PollingMode,
    #[serde(default)]
    pub devices: HashSet<Address>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let interval =
            crate::from_env_or("AGENT_COLLECTOR_MIFLORA_SENSOR_INTERVAL", default_interval)?;
        let devices = std::env::var("AGENT_COLLECTOR_MIFLORA_SENSOR_DEVICES")
            .unwrap_or_default()
            .split(',')
            .filter_map(|value| match Address::from_str(value) {
                Ok(v) => Some(v),
                Err(err) => {
                    tracing::warn!(message = "unable to parse devices address, skipping", address = %value, error = %err);
                    None
                }
            })
            .collect();
        let mode = crate::from_env_or("AGENT_COLLECTOR_MIFLORA_SENSOR_MODE", PollingMode::default)?;
        Ok(Self {
            interval,
            mode,
            devices,
        })
    }

    pub fn build(&self, ctx: &crate::BuildContext) -> Collector {
        Collector {
            adapter: ctx.bluetooth.clone(),
            devices: self.devices.iter().map(|v| bluer::Address(v.0)).collect(),
            interval: Duration::new(self.interval, 0),
            mode: self.mode,
            receiver: ctx.watcher.bluetooth.resubscribe(),
            sender: ctx.sender.clone(),
            history: HashMap::with_capacity(self.devices.len()),
        }
    }
}

pub struct Collector {
    adapter: bluer::Adapter,
    devices: HashSet<bluer::Address>,
    interval: Duration,
    mode: PollingMode,
    receiver: broadcast::Receiver<WatcherEvent>,
    sender: mpsc::Sender<OneOrMany<Metric>>,
    history: HashMap<bluer::Address, u64>,
}

impl Collector {
    #[tracing::instrument(skip(self))]
    async fn collect(&self) {
        let ts = now();
        let available = match self.adapter.device_addresses().await {
            Ok(inner) => inner,
            Err(err) => {
                tracing::warn!(message = "unable to list known addresses", error = %err);
                return;
            }
        };
        for addr in available
            .iter()
            .filter(|d| self.devices.contains(d))
            .filter(|d| {
                self.history
                    .get(*d)
                    .map_or(true, |last| last + self.interval.as_secs() <= ts)
            })
        {
            if let Err(err) = self.handle(*addr, now()).await {
                tracing::warn!(message = "unable to handle bluetooth event", error = %err);
            }
        }
    }

    #[tracing::instrument(skip(self, timestamp))]
    async fn handle(&self, addr: bluer::Address, timestamp: u64) -> anyhow::Result<()> {
        let device = bluer_miflora::Miflora::try_from_adapter(&self.adapter, addr).await?;
        device.connect().await?;
        let system = device.read_system().await?;

        match self.mode {
            PollingMode::History => {
                let history = device.read_historical_values().await?;

                let mut metrics = Vec::with_capacity(history.len() * 4 + 1);
                metrics.push(Metric::new(
                    timestamp,
                    MetricHeader::new(DEVICE_BATTERY).with_tag("address", addr.to_string()),
                    system.battery() as f64,
                ));
                metrics.extend(history.iter().flat_map(|m| {
                    [
                        Metric::new(
                            m.timestamp(),
                            MetricHeader::new(DEVICE_TEMPERATURE)
                                .with_tag("address", addr.to_string()),
                            m.temperature() as f64,
                        ),
                        Metric::new(
                            m.timestamp(),
                            MetricHeader::new(DEVICE_BRIGHTNESS)
                                .with_tag("address", addr.to_string()),
                            m.brightness() as f64,
                        ),
                        Metric::new(
                            m.timestamp(),
                            MetricHeader::new(DEVICE_CONDUCTIVITY)
                                .with_tag("address", addr.to_string()),
                            m.conductivity() as f64,
                        ),
                        Metric::new(
                            m.timestamp(),
                            MetricHeader::new(DEVICE_MOISTURE)
                                .with_tag("address", addr.to_string()),
                            m.moisture() as f64,
                        ),
                    ]
                    .into_iter()
                }));
                self.sender.send_many(metrics).await;

                device.clear_historical_entries().await?;
            }
            PollingMode::Realtime => {
                let realtime = device.read_realtime_values().await?;
                self.sender
                    .send_many(vec![
                        Metric::new(
                            timestamp,
                            MetricHeader::new(DEVICE_BATTERY).with_tag("address", addr.to_string()),
                            system.battery() as f64,
                        ),
                        Metric::new(
                            timestamp,
                            MetricHeader::new(DEVICE_TEMPERATURE)
                                .with_tag("address", addr.to_string()),
                            realtime.temperature() as f64,
                        ),
                        Metric::new(
                            timestamp,
                            MetricHeader::new(DEVICE_BRIGHTNESS)
                                .with_tag("address", addr.to_string()),
                            realtime.brightness() as f64,
                        ),
                        Metric::new(
                            timestamp,
                            MetricHeader::new(DEVICE_CONDUCTIVITY)
                                .with_tag("address", addr.to_string()),
                            realtime.conductivity() as f64,
                        ),
                        Metric::new(
                            timestamp,
                            MetricHeader::new(DEVICE_MOISTURE)
                                .with_tag("address", addr.to_string()),
                            realtime.moisture() as f64,
                        ),
                    ])
                    .await;
            }
        }
        device.disconnect().await?;
        Ok(())
    }
}

impl crate::prelude::Worker for Collector {
    #[tracing::instrument(name = "atc-sensor", skip_all)]
    async fn run(mut self) -> anyhow::Result<()> {
        tracing::info!(message = "starting", devices = ?self.devices);
        let mut interval = tokio::time::interval(self.interval);
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.collect().await;
                    interval.reset();
                }
                res = self.receiver.recv() => {
                    match res {
                        Ok(WatcherEvent::DeviceAdded(addr)) | Ok(WatcherEvent::DeviceChanged(addr, _)) if self.devices.contains(&addr) => {
                            if let Err(err) = self.handle(addr, now()).await {
                                tracing::warn!(message = "unable to handle sensor", address = %addr, error = %err);
                            }
                        }
                        Ok(_) => {}
                        Err(broadcast::error::RecvError::Closed) => return Ok(()),
                        Err(broadcast::error::RecvError::Lagged(err)) => {
                            tracing::warn!(message = "something went wrong with bluetooth events", error = %err);
                        }
                    }
                }
            }
        }
    }
}
