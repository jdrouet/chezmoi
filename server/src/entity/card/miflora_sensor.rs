use std::collections::HashSet;

use chezmoi_entity::metric::MetricHeader;
use chezmoi_ui_static::component::card::miflora_sensor;

use crate::helper::LatestResult;

#[derive(Debug)]
pub(crate) struct Headers {
    temperature: MetricHeader<'static>,
    brightness: MetricHeader<'static>,
    conductivity: MetricHeader<'static>,
    moisture: MetricHeader<'static>,
    battery: MetricHeader<'static>,
}

impl Headers {
    fn from_address(addr: &str) -> Self {
        Self {
            temperature: MetricHeader::new("miflora.temperature")
                .with_tag("address", addr.to_string()),
            brightness: MetricHeader::new("miflora.brightness")
                .with_tag("address", addr.to_string()),
            conductivity: MetricHeader::new("miflora.conductivity")
                .with_tag("address", addr.to_string()),
            moisture: MetricHeader::new("miflora.moisture").with_tag("address", addr.to_string()),
            battery: MetricHeader::new("miflora.battery").with_tag("address", addr.to_string()),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(from = "miflora_sensor::Definition")]
pub struct Config {
    #[serde(flatten)]
    pub inner: miflora_sensor::Definition,
    pub headers: Headers,
}

impl From<miflora_sensor::Definition> for Config {
    fn from(inner: miflora_sensor::Definition) -> Self {
        let headers = Headers::from_address(&inner.address);
        Self { inner, headers }
    }
}

impl Config {
    pub fn latest_filters<'a>(&'a self, list: &mut HashSet<&'a MetricHeader<'a>>) {
        list.insert(&self.headers.temperature);
        list.insert(&self.headers.brightness);
        list.insert(&self.headers.conductivity);
        list.insert(&self.headers.moisture);
        list.insert(&self.headers.battery);
    }

    pub fn build<'a>(&'a self, metrics: &LatestResult) -> miflora_sensor::MifloraSensorCard<'a> {
        miflora_sensor::MifloraSensorCard {
            definition: &self.inner,
            values: miflora_sensor::Values {
                temperature: metrics.find(&self.headers.temperature),
                brightness: metrics.find(&self.headers.brightness),
                conductivity: metrics.find(&self.headers.conductivity),
                moisture: metrics.find(&self.headers.moisture),
                battery: metrics.find(&self.headers.battery),
            },
        }
    }
}
