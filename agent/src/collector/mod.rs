use crate::BuildContext;

#[cfg(feature = "collector-atc-sensor")]
pub mod atc_sensor;
pub mod internal;
pub mod system;

pub mod prelude;

const fn default_false() -> bool {
    false
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Config {
    #[cfg(feature = "collector-atc-sensor")]
    AtcSensor(atc_sensor::Config),
    Internal(internal::Config),
    System(system::Config),
}

impl Config {
    pub fn from_env() -> anyhow::Result<Vec<Self>> {
        let mut result = Vec::new();
        #[cfg(feature = "collector-atc-sensor")]
        if crate::from_env_or("AGENT_COLLECTOR_ATC_SENSOR_ENABLED", default_false)? {
            result.push(Config::AtcSensor(atc_sensor::Config::from_env()?));
        }
        if crate::from_env_or("AGENT_COLLECTOR_INTERNAL_ENABLED", default_false)? {
            result.push(Config::Internal(internal::Config::from_env()?));
        }
        if crate::from_env_or("AGENT_COLLECTOR_SYSTEM_ENABLED", default_false)? {
            result.push(Config::Internal(internal::Config::from_env()?));
        }
        Ok(result)
    }

    pub fn build(&self, ctx: &BuildContext) -> Collector {
        match self {
            #[cfg(feature = "collector-atc-sensor")]
            Self::AtcSensor(inner) => Collector::AtcSensor(inner.build(ctx)),
            Self::Internal(inner) => Collector::Internal(inner.build(ctx)),
            Self::System(inner) => Collector::System(inner.build(ctx)),
        }
    }
}

pub enum Collector {
    #[cfg(feature = "collector-atc-sensor")]
    AtcSensor(atc_sensor::Collector),
    Internal(internal::Collector),
    System(system::Collector),
}

impl crate::prelude::Worker for Collector {
    async fn run(self) -> anyhow::Result<()> {
        match self {
            #[cfg(feature = "collector-atc-sensor")]
            Self::AtcSensor(inner) => inner.run().await,
            Self::Internal(inner) => inner.run().await,
            Self::System(inner) => inner.run().await,
        }
    }
}
