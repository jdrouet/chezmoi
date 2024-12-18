use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::Duration;

use bluer::{DeviceProperty, Uuid};
use chezmoi_entity::address::Address;
use chezmoi_entity::metric::{Metric, MetricHeader};
use chezmoi_entity::{now, OneOrMany};
use tokio::sync::{broadcast, mpsc};

use super::prelude::SenderExt;
use crate::watcher::bluetooth::WatcherEvent;

pub const DEVICE_TEMPERATURE: &str = "atc-thermometer.temperature";
pub const DEVICE_HUMIDITY: &str = "atc-thermometer.humidity";
pub const DEVICE_BATTERY: &str = "atc-thermometer.battery";

pub const fn default_interval() -> u64 {
    60
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_interval")]
    pub interval: u64,
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
        Ok(Self { interval, devices })
    }

    pub fn build(&self, ctx: &crate::BuildContext) -> Collector {
        Collector {
            adapter: ctx.bluetooth.clone(),
            devices: self.devices.iter().map(|v| bluer::Address(v.0)).collect(),
            interval: Duration::new(self.interval, 0),
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

    async fn read_data(&self, addr: bluer::Address, timestamp: u64, data: &[u8]) {
        if let Some(data) = Payload::read(data) {
            self.sender
                .send_many(vec![
                    Metric {
                        timestamp,
                        header: MetricHeader::new(DEVICE_TEMPERATURE)
                            .with_tag("address", addr.to_string()),
                        value: data.temperature as f64,
                    },
                    Metric {
                        timestamp,
                        header: MetricHeader::new(DEVICE_HUMIDITY)
                            .with_tag("address", addr.to_string()),
                        value: data.humidity as f64,
                    },
                    Metric {
                        timestamp,
                        header: MetricHeader::new(DEVICE_BATTERY)
                            .with_tag("address", addr.to_string()),
                        value: data.battery as f64,
                    },
                ])
                .await;
        } else {
            tracing::warn!("invalid service data content");
        }
    }

    async fn read_service_data(
        &self,
        addr: bluer::Address,
        timestamp: u64,
        data: HashMap<Uuid, Vec<u8>>,
    ) -> bool {
        if let Some(data) = data.get(&SERVICE_ID) {
            self.read_data(addr, timestamp, data).await;
            true
        } else {
            tracing::debug!("expected service data not found");
            false
        }
    }

    #[tracing::instrument(skip(self, timestamp))]
    async fn handle(&self, addr: bluer::Address, timestamp: u64) -> anyhow::Result<bool> {
        let device = self.adapter.device(addr)?;
        let data = device.service_data().await?;
        if let Some(data) = data {
            self.read_service_data(addr, timestamp, data).await;
            Ok(true)
        } else {
            tracing::warn!("no service data provided");
            Ok(false)
        }
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
                        Ok(WatcherEvent::DeviceAdded(addr)) if self.devices.contains(&addr) => {
                            let ts = now();
                            match self.handle(addr, ts).await {
                                Ok(true) => {
                                    self.history.insert(addr, ts);
                                }
                                Ok(false) => {},
                                Err(err) => {
                                    tracing::warn!(message = "unable to handle bluetooth event", error = %err);
                                }
                            }
                        }
                        Ok(WatcherEvent::DeviceChanged(addr, DeviceProperty::ServiceData(data))) if self.devices.contains(&addr) => {
                            let ts = now();
                            if self.read_service_data(addr, ts, data).await {
                                self.history.insert(addr, ts);
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
