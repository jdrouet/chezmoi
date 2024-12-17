use std::collections::HashSet;
use std::str::FromStr;

use chezmoi_entity::address::Address;
use chezmoi_entity::metric::{Metric, MetricHeader};
use chezmoi_ui_static::view::dashboard;

mod card;

#[derive(Debug, serde::Deserialize)]
pub struct SectionConfig {
    title: String,
    #[serde(default)]
    cards: Vec<card::CardConfig>,
}

impl SectionConfig {
    fn latest_filters(&self) -> impl Iterator<Item = MetricHeader<'static>> + '_ {
        self.cards.iter().flat_map(|c| c.latest_filters())
    }

    pub fn build<'a>(&'a self, metrics: &[Metric]) -> dashboard::Section<'a> {
        dashboard::Section::new(
            self.title.as_str(),
            self.cards.iter().map(|c| c.build(metrics)).collect(),
        )
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct DashboardConfig {
    #[serde(default)]
    sections: Vec<SectionConfig>,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            sections: vec![SectionConfig {
                title: String::from("Thermometer"),
                cards: vec![
                    card::CardConfig::AtcSensor(card::atc_sensor::Config {
                        name: Some("Living room".into()),
                        address: Address::from_str("A4:C1:38:E1:6F:B2").unwrap(),
                        temperature: card::Range {
                            min: Some(19.0),
                            max: Some(22.0),
                        },
                        humidity: card::Range {
                            min: Some(30.0),
                            max: Some(60.0),
                        },
                        battery: card::Range {
                            min: Some(10.0),
                            max: None,
                        },
                    }),
                    card::CardConfig::AtcSensor(card::atc_sensor::Config {
                        name: Some("Bedroom".into()),
                        address: Address::from_str("A4:C1:38:45:51:3E").unwrap(),
                        temperature: card::Range {
                            min: Some(19.0),
                            max: Some(22.0),
                        },
                        humidity: card::Range {
                            min: Some(30.0),
                            max: Some(60.0),
                        },
                        battery: card::Range {
                            min: Some(10.0),
                            max: None,
                        },
                    }),
                    card::CardConfig::AtcSensor(card::atc_sensor::Config {
                        name: Some("Office".into()),
                        address: Address::from_str("A4:C1:38:1C:02:76").unwrap(),
                        temperature: card::Range {
                            min: Some(19.0),
                            max: Some(22.0),
                        },
                        humidity: card::Range {
                            min: Some(30.0),
                            max: Some(60.0),
                        },
                        battery: card::Range {
                            min: Some(10.0),
                            max: None,
                        },
                    }),
                    card::CardConfig::AtcSensor(card::atc_sensor::Config {
                        name: Some("Outside".into()),
                        address: Address::from_str("A4:C1:38:4E:92:06").unwrap(),
                        temperature: card::Range {
                            min: Some(5.0),
                            max: Some(22.0),
                        },
                        humidity: card::Range {
                            min: Some(30.0),
                            max: Some(60.0),
                        },
                        battery: card::Range {
                            min: Some(10.0),
                            max: None,
                        },
                    }),
                ],
            }],
        }
    }
}

impl DashboardConfig {
    pub fn latest_filters(&self) -> HashSet<MetricHeader<'static>> {
        self.sections
            .iter()
            .flat_map(|s| s.latest_filters())
            .collect()
    }

    pub fn build<'a>(&'a self, metrics: &[Metric]) -> dashboard::DashboardView<'a> {
        dashboard::DashboardView::new(
            "/",
            self.sections.iter().map(|s| s.build(metrics)).collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_deserialize() {
        let _: super::DashboardConfig = serde_json::from_value(serde_json::json!({
            "sections": [
                {
                    "title": "foo",
                    "cards": [
                        {
                            "type": "atc-sensor",
                            "address": "00:00:00:00:00",
                            "temperature": {
                                "min": 12.34,
                                "max": 23.45
                            }
                        }
                    ],
                }
            ]
        }))
        .unwrap();
    }
}
