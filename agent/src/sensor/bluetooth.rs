use bluer::{Address, DeviceEvent, DeviceProperty};
use chezmoi_database::metrics::entity::{Metric, MetricValue};
use chezmoi_database::metrics::{MetricHeader, MetricName, MetricTagValue, MetricTags};
use chezmoi_helper::env::parse_env_or;
use futures::stream::SelectAll;
use futures::{pin_mut, StreamExt};

pub const DEVICE_POWER: &str = "bluetooth.device.power";
pub const DEVICE_VISIBLE: &str = "bluetooth.device.visible";

pub(crate) struct Config {
    enabled: bool,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            enabled: parse_env_or("SENSOR_BLUETOOTH_ENABLED", false)?,
        })
    }

    pub async fn build(self) -> anyhow::Result<Option<Sensor>> {
        if self.enabled {
            let session = bluer::Session::new().await?;
            let adapter = session.default_adapter().await?;

            Ok(Some(Sensor { adapter }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub(crate) struct Sensor {
    adapter: bluer::Adapter,
}

impl Sensor {
    async fn handle_device_added(
        &self,
        hostname: &super::Hostname,
        addr: Address,
        collector: &mut super::Collector,
    ) -> anyhow::Result<()> {
        let now = chezmoi_database::helper::now();
        let device = self.adapter.device(addr)?;
        let device_name = device.name().await?;
        let tags = MetricTags::default()
            .maybe_with(crate::HOSTNAME, hostname.inner())
            .with(crate::ADDRESS, addr.to_string())
            .maybe_with("name", device_name);
        if let Ok(Some(power)) = device.tx_power().await {
            collector.collect(Metric {
                timestamp: now,
                header: MetricHeader {
                    name: MetricName::new(DEVICE_POWER),
                    tags: tags.clone(),
                },
                value: MetricValue::gauge(power as f64),
            });
        }
        collector.collect(Metric {
            timestamp: now,
            header: MetricHeader {
                name: MetricName::new(DEVICE_VISIBLE),
                tags,
            },
            value: MetricValue::bool(true),
        });
        Ok(())
    }

    async fn handle_device_removed(
        &self,
        hostname: &super::Hostname,
        addr: Address,
        collector: &mut super::Collector,
    ) -> anyhow::Result<()> {
        let now = chezmoi_database::helper::now();
        let tags = MetricTags::default()
            .with("address", MetricTagValue::Text(addr.to_string().into()))
            .maybe_with("hostname", hostname.inner().map(MetricTagValue::ArcText));
        collector.collect(Metric {
            timestamp: now,
            header: MetricHeader {
                name: MetricName::new(DEVICE_VISIBLE),
                tags,
            },
            value: MetricValue::bool(false),
        });
        Ok(())
    }

    async fn handle_property_changed(
        &self,
        hostname: &super::Hostname,
        addr: Address,
        _changed: DeviceProperty,
        collector: &mut super::Collector,
    ) -> anyhow::Result<()> {
        self.handle_device_added(hostname, addr, collector).await
    }

    async fn collect(&self, context: &super::Context) -> anyhow::Result<()> {
        let hostname = super::Hostname::default();
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
                            if let Err(error) = self.handle_device_added(&hostname, addr, &mut collector).await {
                                tracing::warn!(message = "unable to handle added device", address = %addr, cause = %error);
                            }
                            if let Ok(device) = self.adapter.device(addr) {
                                let change_events = device.events().await?.map(move |evt| (addr, evt));
                                all_change_events.push(change_events);
                            }
                        }
                        bluer::AdapterEvent::DeviceRemoved(addr) => {
                            if let Err(error) = self.handle_device_removed(&hostname, addr, &mut collector).await {
                                tracing::warn!(message = "unable to handle removed device", address = %addr, cause = %error);
                            }
                        }
                        _ => (),
                    }
                }
                Some((addr, DeviceEvent::PropertyChanged(changed))) = all_change_events.next() => {
                    if let Err(error) = self.handle_property_changed(&hostname, addr, changed, &mut collector).await {
                        tracing::warn!(message = "unable to handle changed property", address = %addr, cause = %error);
                    }
                }
                else => break
            };
            context.sender.send(collector.flush()).await?;
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
