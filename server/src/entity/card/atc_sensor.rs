use std::collections::HashSet;

use chezmoi_entity::metric::MetricHeader;
use chezmoi_ui_static::component::card::atc_sensor;

use crate::helper::LatestResult;

#[derive(Debug)]
pub(crate) struct Headers {
    temperature: MetricHeader<'static>,
    humidity: MetricHeader<'static>,
    battery: MetricHeader<'static>,
}

impl Headers {
    fn from_address(addr: &str) -> Self {
        Self {
            temperature: MetricHeader::new("atc-thermometer.temperature")
                .with_tag("address", addr.to_string()),
            humidity: MetricHeader::new("atc-thermometer.humidity")
                .with_tag("address", addr.to_string()),
            battery: MetricHeader::new("atc-thermometer.battery")
                .with_tag("address", addr.to_string()),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(from = "atc_sensor::Definition")]
pub struct Config {
    #[serde(flatten)]
    pub inner: atc_sensor::Definition,
    pub headers: Headers,
}

impl From<atc_sensor::Definition> for Config {
    fn from(inner: atc_sensor::Definition) -> Self {
        let headers = Headers::from_address(&inner.address);
        Self { inner, headers }
    }
}

impl Config {
    pub fn latest_filters<'a>(&'a self, list: &mut HashSet<&'a MetricHeader<'a>>) {
        list.insert(&self.headers.temperature);
        list.insert(&self.headers.humidity);
        list.insert(&self.headers.temperature);
    }

    pub fn build<'a>(&'a self, metrics: &LatestResult) -> atc_sensor::AtcSensorCard<'a> {
        atc_sensor::AtcSensorCard {
            definition: &self.inner,
            values: atc_sensor::Values {
                temperature: metrics.find(&self.headers.temperature),
                humidity: metrics.find(&self.headers.humidity),
                battery: metrics.find(&self.headers.battery),
            },
        }
    }
}
