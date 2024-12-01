use bluer::{Address, DeviceEvent, DeviceProperty};
use chezmoi_database::metrics::entity::{Metric, MetricValue};
use chezmoi_database::metrics::{MetricHeader, MetricName, MetricTagValue, MetricTags};
use futures::stream::SelectAll;
use futures::{pin_mut, StreamExt};

pub const DEVICE_POWER: &str = "bt_scanner.device.power";
pub const DEVICE_BATTERY: &str = "bt_scanner.device.battery";

#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct Config;

impl Config {
    pub fn build(self, adapter: bluer::Adapter) -> anyhow::Result<Sensor> {
        Ok(Sensor { adapter })
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
        if let Ok(Some(power)) = device.tx_power().await {
            let tags = MetricTags::default()
                .with(crate::ADDRESS, addr.to_string())
                .maybe_with("name", device_name.clone());
            collector.collect(Metric {
                timestamp: chezmoi_database::helper::now(),
                header: MetricHeader {
                    name: MetricName::new(DEVICE_POWER),
                    tags,
                },
                value: MetricValue::gauge(power as f64),
            });
        }
        if let Ok(Some(battery)) = device.battery_percentage().await {
            let tags = MetricTags::default()
                .with(crate::ADDRESS, addr.to_string())
                .maybe_with("name", device_name);
            collector.collect(Metric {
                timestamp: chezmoi_database::helper::now(),
                header: MetricHeader {
                    name: MetricName::new(DEVICE_BATTERY),
                    tags,
                },
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
        collector.collect(Metric {
            timestamp: chezmoi_database::helper::now(),
            header: MetricHeader {
                name: MetricName::new(DEVICE_POWER),
                tags: MetricTags::default()
                    .with("address", MetricTagValue::Text(addr.to_string().into())),
            },
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

    async fn collect(&self, context: &super::Context) -> anyhow::Result<()> {
        self.adapter.set_powered(true).await?;
        self.adapter
            .set_discovery_filter(bluer::DiscoveryFilter::default())
            .await?;
        let device_events = self.adapter.discover_devices().await?;
        pin_mut!(device_events);

        let mut collector = super::Collector::new(super::Cache::default(), 2);
        let mut all_change_events = SelectAll::new();
        while context.state.is_running() {
            tokio::select! {
                Some(event) = device_events.next() => {
                    match event {
                        bluer::AdapterEvent::DeviceAdded(addr) => {
                            if let Err(error) = self.handle_device_added(addr, &mut collector).await {
                                tracing::warn!(message = "unable to handle added device", address = %addr, cause = %error);
                            }
                            if let Ok(device) = self.adapter.device(addr) {
                                let change_events = device.events().await?.map(move |evt| (addr, evt));
                                all_change_events.push(change_events);
                            }
                        }
                        bluer::AdapterEvent::DeviceRemoved(addr) => {
                            if let Err(error) = self.handle_device_removed(addr, &mut collector).await {
                                tracing::warn!(message = "unable to handle removed device", address = %addr, cause = %error);
                            }
                        }
                        _ => (),
                    }
                }
                Some((addr, DeviceEvent::PropertyChanged(changed))) = all_change_events.next() => {
                    if let Err(error) = self.handle_property_changed(addr, changed, &mut collector).await {
                        tracing::warn!(message = "unable to handle changed property", address = %addr, cause = %error);
                    }
                }
                else => break
            };
            context.send_all(collector.flush()).await;
        }
        Ok(())
    }

    pub async fn run(self, context: super::Context) -> anyhow::Result<()> {
        while context.state.is_running() {
            self.collect(&context).await?;
        }
        Ok(())
    }
}
