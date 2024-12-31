use std::sync::Arc;

use chezmoi_entity::metric::{Metric, MetricHeader};

pub mod cache;
pub mod prelude;

pub type AgentMetric = Metric<Arc<MetricHeader<'static>>>;

pub struct BuildContext {
    pub bluetooth: BluetoothBuildContext,
    pub hostname: String,
}

pub struct BluetoothBuildContext {
    pub adapter: bluer::Adapter,
    pub receiver: tokio::sync::broadcast::Receiver<BluetoothEvent>,
}

#[derive(Clone, Debug)]
pub enum BluetoothEvent {
    DeviceAdded(bluer::Address),
    DeviceRemoved(bluer::Address),
    DeviceChanged(bluer::Address, bluer::DeviceProperty),
}

impl BluetoothEvent {
    pub fn address(&self) -> bluer::Address {
        match self {
            Self::DeviceAdded(addr) | Self::DeviceChanged(addr, _) | Self::DeviceRemoved(addr) => {
                *addr
            }
        }
    }
}
