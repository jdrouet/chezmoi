use std::collections::{HashMap, HashSet};
use std::time::Duration;

use bluer::{DeviceProperty, Uuid};
use chezmoi_entity::{now, OneOrMany};
use tokio::sync::{broadcast, mpsc};

use super::helper::CachedSender;
use crate::metric::AgentMetric;
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
    temperature: f32,
    humidity: u8,
    battery: u8,
}

impl Payload {
    fn read(data: &[u8]) -> Option<Self> {
        tracing::trace!(message = "parsing service data", content = ?data, len = data.len());
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
    pub interval: u64,
    #[serde(default)]
    pub devices: HashSet<bluer::Address>,
}

impl Config {
    pub fn build(&self, ctx: &crate::BuildContext) -> Collector {
        Collector {
            adapter: ctx.bluetooth.clone(),
            devices: self.devices.clone(),
            interval: Duration::new(self.interval, 0),
            receiver: ctx.watcher.bluetooth.resubscribe(),
            history: HashMap::with_capacity(self.devices.len()),
        }
    }
}

#[derive(Clone, Debug, Hash)]
pub enum AgentMetricName {
    Temperature,
    Humidity,
    Battery,
}

impl AgentMetricName {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Temperature => "atc-thermometer.temperature",
            Self::Humidity => "atc-thermometer.humidity",
            Self::Battery => "atc-thermometer.battery",
        }
    }
}

impl serde::Serialize for AgentMetricName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Clone, Debug, Hash, serde::Serialize)]
pub struct AgentMetricTags {
    address: bluer::Address,
}

#[derive(Clone, Debug, Hash, serde::Serialize)]
pub struct AgentMetricHeader {
    name: AgentMetricName,
    tags: AgentMetricTags,
}

impl AgentMetricHeader {
    pub const fn new(name: AgentMetricName, address: bluer::Address) -> Self {
        Self {
            name,
            tags: AgentMetricTags { address },
        }
    }
}

pub struct Collector {
    adapter: bluer::Adapter,
    devices: HashSet<bluer::Address>,
    interval: Duration,
    receiver: broadcast::Receiver<WatcherEvent>,
    history: HashMap<bluer::Address, u64>,
}

impl Collector {
    #[tracing::instrument(skip(self, sender))]
    async fn collect(&self, sender: &mut CachedSender) {
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
            if let Err(err) = self.handle(*addr, now(), sender).await {
                tracing::warn!(message = "unable to handle bluetooth event", error = %err);
            }
        }
    }

    async fn read_data(
        &self,
        addr: bluer::Address,
        timestamp: u64,
        data: &[u8],
        sender: &mut CachedSender,
    ) {
        if let Some(data) = Payload::read(data) {
            sender
                .send_many([
                    AgentMetric::new(
                        timestamp,
                        AgentMetricHeader::new(AgentMetricName::Temperature, addr),
                        data.temperature as f64,
                    ),
                    AgentMetric::new(
                        timestamp,
                        AgentMetricHeader::new(AgentMetricName::Humidity, addr),
                        data.humidity as f64,
                    ),
                    AgentMetric::new(
                        timestamp,
                        AgentMetricHeader::new(AgentMetricName::Battery, addr),
                        data.battery as f64,
                    ),
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
        sender: &mut CachedSender,
    ) -> bool {
        if let Some(data) = data.get(&SERVICE_ID) {
            self.read_data(addr, timestamp, data, sender).await;
            true
        } else {
            tracing::debug!("expected service data not found");
            false
        }
    }

    #[tracing::instrument(skip(self, timestamp, sender))]
    async fn handle(
        &self,
        addr: bluer::Address,
        timestamp: u64,
        sender: &mut CachedSender,
    ) -> anyhow::Result<bool> {
        let device = self.adapter.device(addr)?;
        let data = device.service_data().await?;
        if let Some(data) = data {
            self.read_service_data(addr, timestamp, data, sender).await;
            Ok(true)
        } else {
            tracing::warn!("no service data provided");
            Ok(false)
        }
    }
}

impl Collector {
    #[tracing::instrument(name = "atc-sensor", skip_all)]
    pub async fn run(mut self, sender: mpsc::Sender<OneOrMany<AgentMetric>>) -> anyhow::Result<()> {
        tracing::info!(message = "starting", devices = ?self.devices);
        let mut interval = tokio::time::interval(self.interval);
        let mut sender = CachedSender::new(self.devices.len() * 3, self.interval.as_secs(), sender);
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.collect(&mut sender).await;
                    interval.reset();
                }
                res = self.receiver.recv() => {
                    match res {
                        Ok(WatcherEvent::DeviceAdded(addr)) if self.devices.contains(&addr) => {
                            let ts = now();
                            match self.handle(addr, ts, &mut sender).await {
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
                            if self.read_service_data(addr, ts, data, &mut sender).await {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_serialize() {
        let value = serde_json::to_value(&AgentMetric::new(
            1234,
            AgentMetricHeader::new(AgentMetricName::Temperature, bluer::Address::any()),
            12.34,
        ))
        .unwrap();
        assert_eq!(
            value,
            serde_json::json!({
                "timestamp": 1234,
                "name": "atc-thermometer.temperature",
                "tags": {
                    "address": "00:00:00:00:00:00",
                },
                "value": 12.34,
            })
        );
    }
}
