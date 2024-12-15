use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::Duration;

use chezmoi_entity::metric::{Metric, MetricHeader};
use chezmoi_entity::{now, OneOrMany};
use tokio::sync::{broadcast, mpsc};

use super::prelude::SenderExt;
use crate::watcher::bluetooth::WatcherEvent;

pub const DEVICE_TEMPERATURE: &str = "atc-thermometer.temperature";
pub const DEVICE_HUMIDITY: &str = "atc-thermometer.humidity";
pub const DEVICE_BATTERY: &str = "atc-thermometer.battery";

const SERVICE_ID: bluer::Uuid = bluer::Uuid::from_u128(488837762788578050050668711589115);

const TEMPERATURE_INDEX: usize = 6;
const HUMIDITY_INDEX: usize = 8;
const BATTERY_INDEX: usize = 9;

fn read_temperature(data: &[u8]) -> Option<f32> {
    read_f32(data, TEMPERATURE_INDEX)
}

fn read_humidity(data: &[u8]) -> Option<u8> {
    read_u8(data, HUMIDITY_INDEX)
}

fn read_battery(data: &[u8]) -> Option<u8> {
    read_u8(data, BATTERY_INDEX)
}

fn read_u8(data: &[u8], index: usize) -> Option<u8> {
    data.get(index).copied()
}

fn read_f32(data: &[u8], index: usize) -> Option<f32> {
    let value = [*data.get(index)?, *data.get(index + 1)?];
    Some(i16::from_be_bytes(value) as f32 / 10.0)
}

struct Payload {
    pub temperature: f32,
    pub humidity: u8,
    pub battery: u8,
}

impl Payload {
    fn read(data: &[u8]) -> Option<Self> {
        Some(Self {
            temperature: read_temperature(data)?,
            humidity: read_humidity(data)?,
            battery: read_battery(data)?,
        })
    }
}

pub const fn default_interval() -> u64 {
    60
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_interval")]
    interval: u64,
    #[serde(default)]
    devices: HashSet<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let interval = crate::from_env_or("AGENT_COLLECTOR_ATC_SENSOR_INTERVAL", default_interval)?;
        let devices = std::env::var("AGENT_COLLECTOR_ATC_SENSOR_DEVICES")
            .unwrap_or_default()
            .split(',')
            .map(String::from)
            .collect();
        Ok(Self { interval, devices })
    }

    pub fn build(&self, ctx: &crate::BuildContext) -> Collector {
        Collector {
            adapter: ctx.bluetooth.clone(),
            devices: HashSet::from_iter(
                self.devices
                    .iter()
                    .filter_map(|value| bluer::Address::from_str(value.as_str()).ok()),
            ),
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
    async fn collect(&self) {
        let ts = now();
        for addr in self.devices.iter().filter(|d| {
            self.history
                .get(*d)
                .map_or(false, |last| last + self.interval.as_secs() <= ts)
        }) {
            if let Err(err) = self.handle(*addr).await {
                tracing::warn!(message = "unable to handle bluetooth event", error = %err);
            }
        }
    }

    async fn handle(&self, addr: bluer::Address) -> anyhow::Result<()> {
        let device = self.adapter.device(addr)?;
        let timestamp = now();
        let data = device.service_data().await?;
        if let Some(data) =
            data.and_then(|inner| inner.get(&SERVICE_ID).and_then(|data| Payload::read(&data)))
        {
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
        }
        Ok(())
    }
}

impl crate::prelude::Worker for Collector {
    #[tracing::instrument(name = "atc-sensor", skip_all)]
    async fn run(mut self) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(self.interval);
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.collect().await;
                    interval.reset();
                }
                res = self.receiver.recv() => {
                    match res {
                        Ok(event) if self.devices.contains(&event.address()) => {
                            match self.handle(event.address()).await {
                                Ok(()) => {
                                    self.history.insert(event.address(), now());
                                }
                                Err(err) => {
                                    tracing::warn!(message = "unable to handle bluetooth event", error = %err);
                                }
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
