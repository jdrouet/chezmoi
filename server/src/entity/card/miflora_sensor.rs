use std::collections::HashSet;

use chezmoi_entity::metric::{Metric, MetricHeader};
use chezmoi_ui_static::component::card::miflora_sensor;
use chezmoi_ui_static::component::value::TimedValue;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub inner: miflora_sensor::Definition,
}

impl From<miflora_sensor::Definition> for Config {
    fn from(inner: miflora_sensor::Definition) -> Self {
        Self { inner }
    }
}

impl Config {
    pub fn latest_filters<'a>(&'a self, list: &mut HashSet<MetricHeader<'a>>) {
        list.insert(
            MetricHeader::new("miflora.temperature")
                .with_tag("address", self.inner.address.as_str()),
        );
    }

    pub fn build<'a>(&'a self, metrics: &[Metric]) -> miflora_sensor::MifloraSensorCard<'a> {
        miflora_sensor::MifloraSensorCard {
            definition: &self.inner,
            values: miflora_sensor::Values {
                temperature: metrics
                    .iter()
                    .find(|m| {
                        m.header.name.eq("miflora.temperature")
                            && m.header
                                .tags
                                .as_ref()
                                .get("address")
                                .map_or(false, |v| v.eq(self.inner.address.as_str()))
                    })
                    .map(|m| TimedValue {
                        value: m.value,
                        timestamp: m.timestamp,
                    }),
                brightness: None,
                conductivity: None,
                moisture: None,
                battery: metrics
                    .iter()
                    .find(|m| {
                        m.header.name.eq("miflora.battery")
                            && m.header
                                .tags
                                .as_ref()
                                .get("address")
                                .map_or(false, |v| v.eq(self.inner.address.as_str()))
                    })
                    .map(|m| TimedValue {
                        value: m.value,
                        timestamp: m.timestamp,
                    }),
            },
        }
    }
}
