use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use chezmoi_database::helper::now;
use chezmoi_database::metrics::entity::{Metric, MetricValue};
use chezmoi_database::metrics::MetricHeader;
use tokio::sync::broadcast;

use crate::sensor::Collector;
use crate::watcher::bluetooth::WatcherEvent;

#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub devices: HashSet<String>,
    #[serde(default = "crate::sensor::one_hour")]
    pub interval: u64,
}

impl Config {
    pub fn build(&self, adapter: bluer::Adapter) -> Sensor {
        let devices = HashSet::from_iter(
            self.devices
                .iter()
                .filter_map(|addr| bluer::Address::from_str(addr.as_str()).ok()),
        );
        let interval = std::time::Duration::new(self.interval, 0);

        Sensor {
            adapter,
            devices,
            interval,
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

    async fn collect(&self, collector: &mut Collector) {
        let mut missing: HashSet<bluer::Address> = HashSet::from_iter(self.devices.iter().copied());
        let mut retry = 0;
        while !missing.is_empty() && retry < 5 {
            tokio::time::sleep(Duration::new(10, 0)).await;
            for addr in missing.clone() {
                if let Err(err) = self.handle(collector, addr).await {
                    tracing::warn!(message = "unable to handle device", address = %addr, retry = retry, error = %err, cause = ?err.source());
                } else {
                    missing.remove(&addr);
                }
            }
            retry += 1;
        }
    }

    #[tracing::instrument(name = "miflora", skip_all, fields(adapter = %self.adapter.name()))]
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
                    self.collect(&mut collector).await;
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
            ctx.send_all(collector.flush()).await;
        }
        Ok(())
    }
}
