use chezmoi_entity::metric::MetricHeader;
use chezmoi_ui_static::component::card::atc_sensor;

use crate::helper::{QueryCollector, QueryResult};

#[derive(Clone, Debug)]
pub(crate) struct Headers {
    temperature: MetricHeader<'static>,
    humidity: MetricHeader<'static>,
    battery: MetricHeader<'static>,
}

impl Headers {
    pub fn from_address(addr: &str) -> Self {
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

#[derive(Clone, Debug, serde::Deserialize)]
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
    pub fn collect<'a>(&'a self, col: &mut QueryCollector<'a>) {
        col.latest.insert(&self.headers.temperature);
        col.latest.insert(&self.headers.humidity);
        col.latest.insert(&self.headers.battery);
    }

    pub fn build<'a>(&'a self, metrics: &QueryResult) -> atc_sensor::AtcSensorCard<'a> {
        atc_sensor::AtcSensorCard {
            definition: &self.inner,
            values: atc_sensor::Values {
                temperature: metrics.latest.find(&self.headers.temperature),
                humidity: metrics.latest.find(&self.headers.humidity),
                battery: metrics.latest.find(&self.headers.battery),
            },
            with_link: true,
        }
    }
}
