use chezmoi_entity::metric::MetricHeader;
use chezmoi_ui_static::component::card::miflora_sensor;

use crate::helper::{QueryCollector, QueryResult};

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
    pub fn collect<'a>(&'a self, res: &mut QueryCollector<'a>) {
        res.latest.insert(&self.headers.temperature);
        res.latest.insert(&self.headers.brightness);
        res.latest.insert(&self.headers.conductivity);
        res.latest.insert(&self.headers.moisture);
        res.latest.insert(&self.headers.battery);
    }

    pub fn build<'a>(&'a self, metrics: &QueryResult) -> miflora_sensor::MifloraSensorCard<'a> {
        miflora_sensor::MifloraSensorCard {
            definition: &self.inner,
            values: miflora_sensor::Values {
                temperature: metrics.latest.find(&self.headers.temperature),
                brightness: metrics.latest.find(&self.headers.brightness),
                conductivity: metrics.latest.find(&self.headers.conductivity),
                moisture: metrics.latest.find(&self.headers.moisture),
                battery: metrics.latest.find(&self.headers.battery),
            },
        }
    }
}
