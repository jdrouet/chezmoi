use crate::BuildContext;

pub mod atc_sensor;
pub mod internal;

pub mod prelude;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Config {
    AtcSensor(atc_sensor::Config),
    Internal(internal::Config),
}

impl Config {
    pub fn build(&self, ctx: &BuildContext) -> Collector {
        match self {
            Self::AtcSensor(inner) => Collector::AtcSensor(inner.build(ctx)),
            Self::Internal(inner) => Collector::Internal(inner.build(ctx)),
        }
    }
}

pub enum Collector {
    AtcSensor(atc_sensor::Collector),
    Internal(internal::Collector),
}

impl crate::prelude::Worker for Collector {
    async fn run(self) -> anyhow::Result<()> {
        match self {
            Self::AtcSensor(inner) => inner.run().await,
            Self::Internal(inner) => inner.run().await,
        }
    }
}
