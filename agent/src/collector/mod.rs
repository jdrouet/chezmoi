use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc;

use crate::{metric::AgentMetric, BuildContext};

#[cfg(feature = "collector-atc-sensor")]
pub mod atc_sensor;
pub mod internal;
#[cfg(feature = "collector-miflora-sensor")]
pub mod miflora_sensor;
pub mod system;

mod helper;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Config {
    #[cfg(feature = "collector-atc-sensor")]
    AtcSensor(atc_sensor::Config),
    Internal(internal::Config),
    #[cfg(feature = "collector-miflora-sensor")]
    MifloraSensor(miflora_sensor::Config),
    System(system::Config),
}

impl Config {
    pub fn build(&self, ctx: &BuildContext) -> Collector {
        match self {
            #[cfg(feature = "collector-atc-sensor")]
            Self::AtcSensor(inner) => Collector::AtcSensor(inner.build(ctx)),
            Self::Internal(inner) => Collector::Internal(inner.build(ctx)),
            #[cfg(feature = "collector-miflora-sensor")]
            Self::MifloraSensor(inner) => Collector::MifloraSensor(inner.build(ctx)),
            Self::System(inner) => Collector::System(inner.build(ctx)),
        }
    }
}

pub enum Collector {
    #[cfg(feature = "collector-atc-sensor")]
    AtcSensor(atc_sensor::Collector),
    Internal(internal::Collector),
    #[cfg(feature = "collector-miflora-sensor")]
    MifloraSensor(miflora_sensor::Collector),
    System(system::Collector),
}

impl Collector {
    pub async fn run(self, sender: mpsc::Sender<OneOrMany<AgentMetric>>) -> anyhow::Result<()> {
        match self {
            #[cfg(feature = "collector-atc-sensor")]
            Self::AtcSensor(inner) => inner.run(sender).await,
            Self::Internal(inner) => inner.run(sender).await,
            #[cfg(feature = "collector-miflora-sensor")]
            Self::MifloraSensor(inner) => inner.run(sender).await,
            Self::System(inner) => inner.run(sender).await,
        }
    }
}
