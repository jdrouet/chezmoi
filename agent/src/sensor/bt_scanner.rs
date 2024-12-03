use std::sync::Arc;

use bluer::{Address, DeviceProperty};
use chezmoi_database::metrics::entity::{Metric, MetricValue};
use chezmoi_database::metrics::{MetricHeader, MetricTags};
use tokio::sync::broadcast;

use crate::watcher::bluetooth::WatcherEvent;

pub const DEVICE_POWER: &str = "bt_scanner.device.power";
pub const DEVICE_BATTERY: &str = "bt_scanner.device.battery";

#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct Config;

impl Config {
    pub fn build(&self, adapter: bluer::Adapter) -> Sensor {
        Sensor { adapter }
    }
}

#[derive(Debug)]
pub(crate) struct Sensor {
    adapter: bluer::Adapter,
}

impl Sensor {
    async fn handle_device_added(
        &self,
        addr: Address,
        collector: &mut super::Collector,
    ) -> anyhow::Result<()> {
        let device = self.adapter.device(addr)?;
        let device_name = device.name().await?;
        let device_name: Option<Arc<str>> = device_name.map(Arc::from);
        let device_icon = device.icon().await?;
        let device_icon: Option<Arc<str>> = device_icon.map(Arc::from);
        let address: Arc<str> = Arc::from(addr.to_string());
        if let Ok(Some(power)) = device.tx_power().await {
            let tags = MetricTags::default()
                .with(crate::ADDRESS, address.clone())
                .maybe_with("name", device_name.clone())
                .maybe_with("icon", device_icon.clone());
            collector.collect(Metric {
                timestamp: chezmoi_database::helper::now(),
                header: MetricHeader::from((DEVICE_POWER, tags)),
                value: MetricValue::gauge(power as f64),
            });
        }
        if let Ok(Some(battery)) = device.battery_percentage().await {
            let tags = MetricTags::default()
                .with(crate::ADDRESS, address)
                .maybe_with("name", device_name)
                .maybe_with("icon", device_icon);
            collector.collect(Metric {
                timestamp: chezmoi_database::helper::now(),
                header: MetricHeader::from((DEVICE_BATTERY, tags)),
                value: MetricValue::gauge(battery as f64),
            });
        }
        Ok(())
    }

    async fn handle_device_removed(
        &self,
        addr: Address,
        collector: &mut super::Collector,
    ) -> anyhow::Result<()> {
        let tags = MetricTags::default().with("address", addr.to_string());
        collector.collect(Metric {
            timestamp: chezmoi_database::helper::now(),
            header: MetricHeader::from((DEVICE_POWER, tags)),
            value: MetricValue::gauge(0.0),
        });
        Ok(())
    }

    async fn handle_property_changed(
        &self,
        addr: Address,
        _changed: DeviceProperty,
        collector: &mut super::Collector,
    ) -> anyhow::Result<()> {
        self.handle_device_added(addr, collector).await
    }

    async fn handle_event(&self, event: WatcherEvent, collector: &mut super::Collector) {
        let res = match event {
            WatcherEvent::DeviceAdded(addr) => self.handle_device_added(addr, collector).await,
            WatcherEvent::DeviceRemoved(addr) => self.handle_device_removed(addr, collector).await,
            WatcherEvent::DeviceChanged(addr, changed) => {
                self.handle_property_changed(addr, changed, collector).await
            }
        };
        if let Err(err) = res {
            tracing::error!(message = "unable to handle bluetooth event", error = %err);
        }
    }

    pub async fn run(
        self,
        ctx: super::Context,
        mut recv: broadcast::Receiver<WatcherEvent>,
    ) -> anyhow::Result<()> {
        let mut collector = super::Collector::new(super::Cache::default(), 2);
        while ctx.state.is_running() {
            match recv.recv().await {
                Ok(event) => self.handle_event(event, &mut collector).await,
                Err(broadcast::error::RecvError::Lagged(count)) => {
                    tracing::warn!(message = "bluetooth events got lost", count = %count);
                }
                Err(broadcast::error::RecvError::Closed) => {
                    return Ok(());
                }
            };
        }
        Ok(())
    }
}
