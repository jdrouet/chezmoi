use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::Duration;

use anyhow::Context;
use chezmoi_entity::address::Address;
use chezmoi_entity::metric::{Metric, MetricHeader};
use chezmoi_entity::{now, OneOrMany};
use tokio::sync::{broadcast, mpsc};

use super::helper::CachedSender;
use crate::helper::cache::Cache;
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
    pub fn build(&self, ctx: &crate::BuildContext) -> Collector {
        Collector {
            adapter: ctx.bluetooth.clone(),
            devices: self.devices.iter().map(|v| bluer::Address(v.0)).collect(),
            interval: Duration::new(self.interval, 0),
            mode: self.mode,
            receiver: ctx.watcher.bluetooth.resubscribe(),
        }
    }
}

const ERROR_DELAY: u64 = 60;

fn error_delay(count: usize) -> u64 {
    (count as u64).max(10) * ERROR_DELAY
}

struct LastState {
    error_count: usize,
    timestamp: u64,
}

impl LastState {
    fn should_handle(&self, ttl: u64, now: u64) -> bool {
        let delay = if self.error_count > 0 {
            error_delay(self.error_count)
        } else {
            ttl
        };
        self.timestamp * delay < now
    }
}

pub struct LocalContext {
    inner: HashMap<bluer::Address, LastState>,
}

impl LocalContext {
    fn new(devices: impl Iterator<Item = bluer::Address>) -> Self {
        Self {
            inner: HashMap::from_iter(devices.map(|addr| {
                (
                    addr,
                    LastState {
                        error_count: 0,
                        timestamp: 0,
                    },
                )
            })),
        }
    }

    fn should_handle(&self, ttl: u64, addr: &bluer::Address, now: u64) -> bool {
        let Some(state) = self.inner.get(addr) else {
            tracing::trace!(message = "unable to find device in context", address = %addr);
            return true;
        };

        return state.should_handle(ttl, now);
    }

    fn on_success(&mut self, addr: bluer::Address, now: u64) {
        self.inner.insert(
            addr,
            LastState {
                error_count: 0,
                timestamp: now,
            },
        );
    }

    fn on_error(&mut self, addr: bluer::Address, now: u64) {
        self.inner
            .entry(addr)
            .and_modify(|v| {
                v.error_count += 1;
                v.timestamp = now;
            })
            .or_insert(LastState {
                error_count: 1,
                timestamp: now,
            });
    }
}

pub struct Collector {
    adapter: bluer::Adapter,
    devices: HashSet<bluer::Address>,
    interval: Duration,
    mode: PollingMode,
    receiver: broadcast::Receiver<WatcherEvent>,
}

impl Collector {
    #[tracing::instrument(skip(self, ctx, sender))]
    async fn collect(&self, ctx: &mut LocalContext, sender: &mut CachedSender) {
        for addr in self.devices.iter() {
            self.try_handle(ctx, *addr, now(), sender).await;
        }
    }

    #[tracing::instrument(skip(self, ctx, timestamp, sender))]
    async fn try_handle(
        &self,
        ctx: &mut LocalContext,
        addr: bluer::Address,
        timestamp: u64,
        sender: &mut CachedSender,
    ) {
        if !ctx.should_handle(self.interval.as_secs(), &addr, timestamp) {
            tracing::trace!(message = "the device has already been handled recently");
            return;
        }

        match self.handle(addr, timestamp, sender).await {
            Ok(_) => {
                tracing::trace!("device handled successfully");
                ctx.on_success(addr, timestamp);
            }
            Err(err) => {
                ctx.on_error(addr, timestamp);
                tracing::warn!(message = "unable to handle sensor", error = %err, source = ?err.source());
            }
        }
    }

    async fn handle(
        &self,
        addr: bluer::Address,
        timestamp: u64,
        sender: &mut CachedSender,
    ) -> anyhow::Result<()> {
        let device = bluer_miflora::Miflora::try_from_adapter(&self.adapter, addr)
            .await
            .context("getting device from adapter")?;
        device.connect().await.context("connecting")?;
        let system = device.read_system().await.context("reading system")?;

        match self.mode {
            PollingMode::History => {
                let history = device
                    .read_historical_values()
                    .await
                    .context("reading historical values")?;

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
                sender.send_many(metrics).await;

                device
                    .clear_historical_entries()
                    .await
                    .context("clearing historical values")?;
            }
            PollingMode::Realtime => {
                let realtime = device
                    .read_realtime_values()
                    .await
                    .context("reading realtime values")?;
                sender
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
        device.disconnect().await.context("disconnecting")?;
        Ok(())
    }
}

impl Collector {
    #[tracing::instrument(name = "miflora-sensor", skip_all)]
    pub async fn run(mut self, sender: mpsc::Sender<OneOrMany<Metric>>) -> anyhow::Result<()> {
        tracing::info!(message = "starting", devices = ?self.devices);
        let mut interval = tokio::time::interval(self.interval);
        let mut sender = CachedSender::new(
            Cache::new(self.devices.len() * 5, self.interval.as_secs()),
            sender,
        );
        let mut ctx = LocalContext::new(self.devices.iter().copied());
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.collect(&mut ctx, &mut sender).await;
                    interval.reset();
                }
                res = self.receiver.recv() => {
                    match res {
                        Ok(WatcherEvent::DeviceAdded(addr)) | Ok(WatcherEvent::DeviceChanged(addr, _)) if self.devices.contains(&addr) => {
                            self.try_handle(&mut ctx, addr, now(), &mut sender).await;
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
