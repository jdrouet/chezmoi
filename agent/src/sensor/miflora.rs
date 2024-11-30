use crate::sensor::Collector;
use chezmoi_database::{
    helper::now,
    metrics::{
        entity::{Metric, MetricValue},
        MetricHeader,
    },
};
use chezmoi_helper::env::parse_env_or;
use std::{collections::HashSet, str::FromStr, sync::Arc, time::Duration};

use super::ONE_HOUR;

pub(crate) struct Config {
    pub enabled: bool,
    pub devices: HashSet<String>,
    pub interval: u64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            enabled: parse_env_or("SENSOR_MIFLORA_ENABLED", false)?,
            devices: HashSet::from_iter(
                std::env::var("SENSOR_MIFLORA_DEVICES")
                    .ok()
                    .unwrap_or_default()
                    .split(",")
                    .map(String::from),
            ),
            interval: parse_env_or("SENSOR_MIFLORA_INTERVAL", ONE_HOUR)?,
        })
    }

    pub async fn build(self, adapter: bluer::Adapter) -> anyhow::Result<Option<Sensor>> {
        if self.enabled {
            let devices = HashSet::from_iter(
                self.devices
                    .iter()
                    .filter_map(|addr| bluer::Address::from_str(addr).ok()),
            );
            let interval = std::time::Duration::new(self.interval, 0);
            Ok(Some(Sensor {
                adapter,
                devices,
                interval,
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub(crate) struct Sensor {
    adapter: bluer::Adapter,
    devices: HashSet<bluer::Address>,
    interval: Duration,
}

impl Sensor {
    #[tracing::instrument(skip_all, fields(addr = %addr))]
    async fn handle(&self, collector: &mut Collector, addr: bluer::Address) -> anyhow::Result<()> {
        let now = now();
        let device = bluer_miflora::Miflora::try_from_adapter(&self.adapter, addr).await?;
        device.connect().await?;
        let address: Arc<str> = Arc::from(addr.to_string());
        let system = device.read_system().await?;
        collector.collect(Metric {
            timestamp: now,
            header: MetricHeader::new("miflora.battery").with_tag(crate::ADDRESS, address.clone()),
            value: MetricValue::gauge(system.battery() as f64),
        });
        let entry = device.read_realtime_values().await?;
        collector.collect(Metric {
            timestamp: now,
            header: MetricHeader::new("miflora.temperature")
                .with_tag(crate::ADDRESS, address.clone()),
            value: MetricValue::gauge((entry.temperature() as f64) * 0.1),
        });
        collector.collect(Metric {
            timestamp: now,
            header: MetricHeader::new("miflora.brightness")
                .with_tag(crate::ADDRESS, address.clone()),
            value: MetricValue::gauge(entry.brightness() as f64),
        });
        collector.collect(Metric {
            timestamp: now,
            header: MetricHeader::new("miflora.moisture").with_tag(crate::ADDRESS, address.clone()),
            value: MetricValue::gauge(entry.moisture() as f64),
        });
        collector.collect(Metric {
            timestamp: now,
            header: MetricHeader::new("miflora.conductivity")
                .with_tag(crate::ADDRESS, address.clone()),
            value: MetricValue::gauge((entry.conductivity() as f64) * 0.0001),
        });

        // let history = device.read_historical_values().await?;
        // for entry in history {
        //     collector.collect(Metric {
        //         timestamp: entry.timestamp(),
        //         header: MetricHeader::new("miflora.temperature")
        //             .with_tag(crate::ADDRESS, address.clone()),
        //         value: MetricValue::gauge((entry.temperature() as f64) * 0.1),
        //     });
        //     collector.collect(Metric {
        //         timestamp: entry.timestamp(),
        //         header: MetricHeader::new("miflora.brightness")
        //             .with_tag(crate::ADDRESS, address.clone()),
        //         value: MetricValue::gauge(entry.brightness() as f64),
        //     });
        //     collector.collect(Metric {
        //         timestamp: entry.timestamp(),
        //         header: MetricHeader::new("miflora.moisture")
        //             .with_tag(crate::ADDRESS, address.clone()),
        //         value: MetricValue::gauge(entry.moisture() as f64),
        //     });
        //     collector.collect(Metric {
        //         timestamp: entry.timestamp(),
        //         header: MetricHeader::new("miflora.conductivity")
        //             .with_tag(crate::ADDRESS, address.clone()),
        //         value: MetricValue::gauge(entry.conductivity() as f64),
        //     });
        // }
        // device.clear_historical_entries().await?;
        device.disconnect().await?;
        Ok(())
    }

    async fn execute(&self, ctx: &super::Context) {
        tracing::debug!("starting scan");
        if let Err(err) = self
            .adapter
            .set_discovery_filter(bluer::DiscoveryFilter::default())
            .await
        {
            tracing::error!(message = "unable to set discovery filter", error = %err);
        }
        let _stream = self.adapter.discover_devices().await;

        let mut collector = Collector::default();
        let mut missing: HashSet<bluer::Address> = HashSet::from_iter(self.devices.iter().copied());
        let mut retry = 0;
        while !missing.is_empty() && retry < 5 {
            tokio::time::sleep(Duration::new(10, 0)).await;
            for addr in missing.clone() {
                if let Err(err) = self.handle(&mut collector, addr).await {
                    tracing::warn!(message = "unable to handle device", address = %addr, retry = retry, error = %err, cause = ?err.source());
                } else {
                    missing.remove(&addr);
                }
            }
            retry += 1;
        }
        ctx.send_all(collector.flush()).await;
    }

    #[tracing::instrument(name = "miflora", skip_all, fields(adapter = %self.adapter.name()))]
    pub async fn run(self, ctx: super::Context) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(self.interval);
        while ctx.state.is_running() {
            interval.tick().await;
            self.execute(&ctx).await;
        }
        Ok(())
    }
}
