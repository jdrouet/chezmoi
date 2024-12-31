use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use chezmoi_entity::metric::MetricHeader;
use chezmoi_entity::now;
use chezmoi_sensor_prelude::agent::cache::CachedSender;
use chezmoi_sensor_prelude::agent::prelude::SensorSender;
use chezmoi_sensor_prelude::agent::{AgentMetric, BluetoothEvent, BuildContext};
use tokio::sync::broadcast;

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
    pub devices: HashSet<bluer::Address>,
}

impl chezmoi_sensor_prelude::agent::prelude::Config for Config {
    type Output = Sensor;

    async fn build(&self, ctx: &BuildContext) -> anyhow::Result<Sensor> {
        Ok(Sensor {
            adapter: ctx.bluetooth.adapter.clone(),
            devices: self
                .devices
                .iter()
                .map(|v| (*v, Headers::new(v.to_string())))
                .collect(),
            interval: Duration::new(self.interval, 0),
            mode: self.mode,
            receiver: ctx.bluetooth.receiver.resubscribe(),
        })
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

        state.should_handle(ttl, now)
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

struct Headers {
    temperature: Arc<MetricHeader<'static>>,
    brightness: Arc<MetricHeader<'static>>,
    conductivity: Arc<MetricHeader<'static>>,
    moisture: Arc<MetricHeader<'static>>,
    battery: Arc<MetricHeader<'static>>,
}

impl Headers {
    pub fn new(addr: String) -> Self {
        Self {
            temperature: Arc::new(
                MetricHeader::new(crate::TEMPERATURE).with_tag("address", addr.clone()),
            ),
            brightness: Arc::new(
                MetricHeader::new(crate::BRIGHTNESS).with_tag("address", addr.clone()),
            ),
            conductivity: Arc::new(
                MetricHeader::new(crate::CONDUCTIVITY).with_tag("address", addr.clone()),
            ),
            moisture: Arc::new(
                MetricHeader::new(crate::MOISTURE).with_tag("address", addr.clone()),
            ),
            battery: Arc::new(MetricHeader::new(crate::BATTERY).with_tag("address", addr.clone())),
        }
    }
}

pub struct Sensor {
    adapter: bluer::Adapter,
    devices: HashMap<bluer::Address, Headers>,
    interval: Duration,
    mode: PollingMode,
    receiver: broadcast::Receiver<BluetoothEvent>,
}

impl Sensor {
    #[tracing::instrument(skip(self, ctx, sender))]
    async fn collect(&self, ctx: &mut LocalContext, sender: &mut CachedSender) {
        for (addr, headers) in self.devices.iter() {
            self.try_handle(ctx, *addr, headers, now(), sender).await;
        }
    }

    #[tracing::instrument(skip(self, ctx, headers, timestamp, sender))]
    async fn try_handle(
        &self,
        ctx: &mut LocalContext,
        addr: bluer::Address,
        headers: &Headers,
        timestamp: u64,
        sender: &mut CachedSender,
    ) {
        if !ctx.should_handle(self.interval.as_secs(), &addr, timestamp) {
            tracing::trace!(message = "the device has already been handled recently");
            return;
        }

        match self.handle(addr, headers, timestamp, sender).await {
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
        headers: &Headers,
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
                metrics.push(AgentMetric::new(
                    timestamp,
                    headers.battery.clone(),
                    system.battery() as f64,
                ));
                metrics.extend(history.iter().flat_map(|m| {
                    [
                        AgentMetric::new(
                            m.timestamp(),
                            headers.temperature.clone(),
                            (m.temperature() as f64) * 0.1,
                        ),
                        AgentMetric::new(
                            m.timestamp(),
                            headers.brightness.clone(),
                            m.brightness() as f64,
                        ),
                        AgentMetric::new(
                            m.timestamp(),
                            headers.conductivity.clone(),
                            (m.conductivity() as f64) * 0.0001,
                        ),
                        AgentMetric::new(
                            m.timestamp(),
                            headers.moisture.clone(),
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
                    .send_many([
                        AgentMetric::new(
                            timestamp,
                            headers.battery.clone(),
                            system.battery() as f64,
                        ),
                        AgentMetric::new(
                            timestamp,
                            headers.temperature.clone(),
                            (realtime.temperature() as f64) * 0.1,
                        ),
                        AgentMetric::new(
                            timestamp,
                            headers.brightness.clone(),
                            realtime.brightness() as f64,
                        ),
                        AgentMetric::new(
                            timestamp,
                            headers.conductivity.clone(),
                            (realtime.conductivity() as f64) * 0.0001,
                        ),
                        AgentMetric::new(
                            timestamp,
                            headers.moisture.clone(),
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

impl chezmoi_sensor_prelude::agent::prelude::Sensor for Sensor {
    #[tracing::instrument(name = "miflora-sensor", skip_all)]
    async fn run(mut self, sender: SensorSender) -> anyhow::Result<()> {
        tracing::info!(message = "starting", devices = ?self.devices.keys());
        let mut interval = tokio::time::interval(self.interval);
        let mut sender = CachedSender::new(self.devices.len() * 5, self.interval.as_secs(), sender);
        let mut ctx = LocalContext::new(self.devices.keys().copied());
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.collect(&mut ctx, &mut sender).await;
                    interval.reset();
                }
                res = self.receiver.recv() => {
                    match res {
                        Ok(BluetoothEvent::DeviceAdded(addr)) | Ok(BluetoothEvent::DeviceChanged(addr, _)) => {
                            if let Some(headers) = self.devices.get(&addr) {
                                self.try_handle(&mut ctx, addr, headers, now(), &mut sender).await;
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
