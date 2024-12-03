use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

use bluer::Address;
use chezmoi_database::helper::now;
use chezmoi_database::metrics::entity::{Metric, MetricValue};
use chezmoi_database::metrics::MetricHeader;
use tokio::sync::broadcast;

use crate::sensor::Collector;
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

#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub devices: HashSet<String>,
    #[serde(default = "crate::sensor::one_hour")]
    pub interval: u64,
}

impl Config {
    pub fn build(&self, adapter: bluer::Adapter) -> Sensor {
        Sensor {
            adapter,
            devices: HashSet::from_iter(
                self.devices
                    .iter()
                    .filter_map(|addr| Address::from_str(addr.as_str()).ok()),
            ),
            interval: std::time::Duration::new(self.interval, 0),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Sensor {
    adapter: bluer::Adapter,
    devices: HashSet<Address>,
    interval: std::time::Duration,
}

impl Sensor {
    #[tracing::instrument(skip_all, fields(addr = %addr))]
    async fn handle(&self, collector: &mut Collector, addr: bluer::Address) -> anyhow::Result<()> {
        let current_ts = now();
        let device = self.adapter.device(addr)?;
        let data = device.service_data().await?;
        let address: Arc<str> = Arc::from(addr.to_string());
        if let Some(data) =
            data.and_then(|inner| inner.get(&SERVICE_ID).and_then(|data| Payload::read(&data)))
        {
            collector.collect(Metric {
                timestamp: current_ts,
                header: MetricHeader::new(DEVICE_TEMPERATURE).with_tag("address", address.clone()),
                value: MetricValue::gauge(data.temperature as f64),
            });
            collector.collect(Metric {
                timestamp: current_ts,
                header: MetricHeader::new(DEVICE_HUMIDITY).with_tag("address", address.clone()),
                value: MetricValue::gauge(data.humidity as f64),
            });
            collector.collect(Metric {
                timestamp: current_ts,
                header: MetricHeader::new(DEVICE_BATTERY).with_tag("address", address),
                value: MetricValue::gauge(data.battery as f64),
            });
        }
        Ok(())
    }

    async fn collect(&self, ctx: &super::Context, collector: &mut Collector) {
        let mut missing: HashSet<Address> = self.devices.clone();
        let mut retry = 0;
        while !missing.is_empty() && retry < 5 {
            tokio::time::sleep(std::time::Duration::new(10, 0)).await;
            for addr in missing.clone() {
                if let Err(err) = self.handle(collector, addr).await {
                    tracing::warn!(message = "unable to handle device", address = %addr, retry = retry, error = %err, cause = ?err.source());
                } else {
                    missing.remove(&addr);
                }
            }
            retry += 1;
        }
        ctx.send_all(collector.flush()).await;
    }

    #[tracing::instrument(name = "atc_thermometer", skip_all, fields(adapter = %self.adapter.name()))]
    pub async fn run(
        self,
        ctx: super::Context,
        mut rcv: broadcast::Receiver<WatcherEvent>,
    ) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(self.interval);
        let mut collector = Collector::default();

        while ctx.state.is_running() {
            tokio::select! {
                _ = interval.tick() => {
                    self.collect(&ctx, &mut collector).await;
                }
                res = rcv.recv() => {
                    match res {
                        Ok(WatcherEvent::DeviceAdded(addr)) | Ok(WatcherEvent::DeviceChanged(addr, _)) => {
                            if let Err(err) = self.handle(&mut collector, addr).await {
                                tracing::warn!(message = "unable to handle device", address = %addr, error = %err, cause = ?err.source());
                            }
                        }
                        Ok(_) => {}
                        Err(broadcast::error::RecvError::Lagged(count)) => {
                            tracing::warn!(message = "bluetooth events got lost", count = %count);
                        }
                        Err(broadcast::error::RecvError::Closed) => break
                    }
                }
                else => break
            }
        }
        Ok(())
    }
}
