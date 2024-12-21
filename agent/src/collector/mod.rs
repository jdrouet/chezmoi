use crate::BuildContext;

#[cfg(feature = "collector-atc-sensor")]
pub mod atc_sensor;
pub mod internal;
#[cfg(feature = "collector-miflora-sensor")]
pub mod miflora_sensor;
pub mod system;

pub mod prelude;

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

impl crate::prelude::Worker for Collector {
    async fn run(self) -> anyhow::Result<()> {
        match self {
            #[cfg(feature = "collector-atc-sensor")]
            Self::AtcSensor(inner) => inner.run().await,
            Self::Internal(inner) => inner.run().await,
            #[cfg(feature = "collector-miflora-sensor")]
            Self::MifloraSensor(inner) => inner.run().await,
            Self::System(inner) => inner.run().await,
        }
    }
}
