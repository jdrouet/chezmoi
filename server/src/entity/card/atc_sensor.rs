use chezmoi_entity::metric::Metric;
use chezmoi_ui_static::component::card::atc_sensor::AtcSensorCard;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    name: Option<String>,
    address: bluer::Address,
    #[serde(default)]
    temperature: super::Range,
    #[serde(default)]
    humidity: super::Range,
    #[serde(default)]
    battery: super::Range,
}

impl Config {
    pub fn build<'a>(&'a self, metrics: &[Metric]) -> AtcSensorCard<'a> {
        AtcSensorCard {
            name: self.name.as_deref(),
            address: self.address.to_string(),
            temperature_definition: self.temperature.into(),
            temperature: None,
            humidity_definition: self.humidity.into(),
            humidity: None,
            battery_definition: self.battery.into(),
            battery: None,
        }
    }
}
