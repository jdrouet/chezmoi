use chezmoi_entity::address::Address;
use chezmoi_entity::metric::{Metric, MetricHeader};
use chezmoi_ui_static::component::card::atc_sensor::AtcSensorCard;
use chezmoi_ui_static::component::value_cell;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub name: Option<String>,
    pub address: Address,
    #[serde(default)]
    pub temperature: super::Range,
    #[serde(default)]
    pub humidity: super::Range,
    #[serde(default)]
    pub battery: super::Range,
}

impl Config {
    pub fn latest_filters(&self) -> impl Iterator<Item = MetricHeader<'static>> {
        [
            MetricHeader::new("atc-thermometer.temperature")
                .with_tag("address", self.address.to_string()),
            MetricHeader::new("atc-thermometer.humidity")
                .with_tag("address", self.address.to_string()),
            MetricHeader::new("atc-thermometer.battery")
                .with_tag("address", self.address.to_string()),
        ]
        .into_iter()
    }

    pub fn build<'a>(&'a self, metrics: &[Metric]) -> AtcSensorCard<'a> {
        let address = self.address.to_string();
        AtcSensorCard {
            name: self.name.as_deref(),
            address: self.address.to_string(),
            temperature_definition: self.temperature.into(),
            temperature: metrics
                .iter()
                .find(|m| {
                    m.header.name.eq("atc-thermometer.temperature")
                        && m.header
                            .tags
                            .as_ref()
                            .get("address")
                            .map_or(false, |v| v.eq(address.as_str()))
                })
                .map(|m| value_cell::Value {
                    value: m.value,
                    timestamp: m.timestamp,
                }),
            humidity_definition: self.humidity.into(),
            humidity: metrics
                .iter()
                .find(|m| {
                    m.header.name.eq("atc-thermometer.humidity")
                        && m.header
                            .tags
                            .as_ref()
                            .get("address")
                            .map_or(false, |v| v.eq(address.as_str()))
                })
                .map(|m| value_cell::Value {
                    value: m.value,
                    timestamp: m.timestamp,
                }),
            battery_definition: self.battery.into(),
            battery: metrics
                .iter()
                .find(|m| {
                    m.header.name.eq("atc-thermometer.battery")
                        && m.header
                            .tags
                            .as_ref()
                            .get("address")
                            .map_or(false, |v| v.eq(address.as_str()))
                })
                .map(|m| value_cell::Value {
                    value: m.value,
                    timestamp: m.timestamp,
                }),
        }
    }
}
