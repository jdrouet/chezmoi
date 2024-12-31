use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use bluer::{DeviceProperty, Uuid};
use chezmoi_entity::metric::MetricHeader;
use chezmoi_entity::now;
use chezmoi_sensor_prelude::agent::cache::CachedSender;
use chezmoi_sensor_prelude::agent::prelude::SensorSender;
use chezmoi_sensor_prelude::agent::{AgentMetric, BluetoothEvent, BuildContext};
use tokio::sync::broadcast;

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

impl chezmoi_sensor_prelude::agent::prelude::Config for Config {
    type Output = Sensor;

    async fn build(&self, ctx: &BuildContext) -> anyhow::Result<Sensor> {
        Ok(Sensor {
            headers: self
                .devices
                .iter()
                .copied()
                .map(|addr| (addr, Headers::new(addr.to_string())))
                .collect(),
            adapter: ctx.bluetooth.adapter.clone(),
            devices: self.devices.clone(),
            interval: Duration::new(self.interval, 0),
            receiver: ctx.bluetooth.receiver.resubscribe(),
            history: HashMap::with_capacity(self.devices.len()),
        })
    }
}

pub struct Headers {
    temperature: Arc<MetricHeader<'static>>,
    humidity: Arc<MetricHeader<'static>>,
    battery: Arc<MetricHeader<'static>>,
}

impl Headers {
    pub fn new(address: String) -> Self {
        Self {
            temperature: Arc::new(
                MetricHeader::new(crate::TEMPERATURE).with_tag("address", address.to_string()),
            ),
            humidity: Arc::new(
                MetricHeader::new(crate::HUMIDITY).with_tag("address", address.to_string()),
            ),
            battery: Arc::new(MetricHeader::new(crate::BATTERY).with_tag("address", address)),
        }
    }
}

pub struct Sensor {
    headers: HashMap<bluer::Address, Headers>,
    adapter: bluer::Adapter,
    devices: HashSet<bluer::Address>,
    interval: Duration,
    receiver: broadcast::Receiver<BluetoothEvent>,
    history: HashMap<bluer::Address, u64>,
}

impl Sensor {
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
        for (addr, headers) in available
            .iter()
            .filter(|d| self.devices.contains(d))
            .filter(|d| {
                self.history
                    .get(*d)
                    .map_or(true, |last| last + self.interval.as_secs() <= ts)
            })
            .filter_map(|d| self.headers.get_key_value(d))
        {
            if let Err(err) = self.handle(*addr, now(), headers, sender).await {
                tracing::warn!(message = "unable to handle bluetooth event", error = %err);
            }
        }
    }

    async fn read_data(
        &self,
        timestamp: u64,
        headers: &Headers,
        data: &[u8],
        sender: &mut CachedSender,
    ) {
        if let Some(data) = Payload::read(data) {
            sender
                .send_many([
                    AgentMetric::new(
                        timestamp,
                        headers.temperature.clone(),
                        data.temperature as f64,
                    ),
                    AgentMetric::new(timestamp, headers.humidity.clone(), data.humidity as f64),
                    AgentMetric::new(timestamp, headers.battery.clone(), data.battery as f64),
                ])
                .await;
        } else {
            tracing::warn!("invalid service data content");
        }
    }

    async fn read_service_data(
        &self,
        timestamp: u64,
        headers: &Headers,
        data: HashMap<Uuid, Vec<u8>>,
        sender: &mut CachedSender,
    ) -> bool {
        if let Some(data) = data.get(&SERVICE_ID) {
            self.read_data(timestamp, headers, data, sender).await;
            true
        } else {
            tracing::debug!("expected service data not found");
            false
        }
    }

    #[tracing::instrument(skip(self, timestamp, headers, sender))]
    async fn handle(
        &self,
        addr: bluer::Address,
        timestamp: u64,
        headers: &Headers,
        sender: &mut CachedSender,
    ) -> anyhow::Result<bool> {
        let device = self.adapter.device(addr)?;
        let data = device.service_data().await?;
        if let Some(data) = data {
            self.read_service_data(timestamp, headers, data, sender)
                .await;
            Ok(true)
        } else {
            tracing::warn!("no service data provided");
            Ok(false)
        }
    }
}

impl Sensor {
    pub async fn run(mut self, sender: SensorSender) -> anyhow::Result<()> {
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
                        Ok(BluetoothEvent::DeviceAdded(addr)) => {
                            let Some(header) = self.headers.get(&addr) else {
                                continue;
                            };
                            let ts = now();
                            match self.handle(addr, ts, header, &mut sender).await {
                                Ok(true) => {
                                    self.history.insert(addr, ts);
                                }
                                Ok(false) => {},
                                Err(err) => {
                                    tracing::warn!(message = "unable to handle bluetooth event", error = %err);
                                }
                            }
                        }
                        Ok(BluetoothEvent::DeviceChanged(addr, DeviceProperty::ServiceData(data))) => {
                            let Some(header) = self.headers.get(&addr) else {
                                continue;
                            };
                            let ts = now();
                            if self.read_service_data(ts, header, data, &mut sender).await {
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
