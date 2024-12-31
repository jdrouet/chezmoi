use std::collections::HashMap;

use card::CardConfig;
use chezmoi_entity::metric::MetricHeader;
use chezmoi_ui_static::component::card::line_chart;
use chezmoi_ui_static::component::range::Range;
use chezmoi_ui_static::view::dashboard;

use crate::helper::{QueryCollector, QueryResult};

pub mod card;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct SectionConfig {
    pub title: String,
    #[serde(default)]
    pub cards: Vec<card::CardConfig>,
}

impl SectionConfig {
    fn collect<'a>(&'a self, collector: &mut QueryCollector<'a>) {
        self.cards.iter().for_each(|c| c.collect(collector));
    }

    pub fn build<'a>(&'a self, res: &QueryResult) -> dashboard::Section<'a> {
        dashboard::Section::new(
            self.title.as_str(),
            self.cards.iter().map(|c| c.build(res)).collect(),
        )
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DashboardConfig {
    #[serde(default)]
    pub sections: Vec<SectionConfig>,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            sections: vec![
                SectionConfig {
                    title: String::from("Thermometer"),
                    cards: vec![
                        card::CardConfig::AtcSensor(
                            chezmoi_ui_static::component::card::atc_sensor::Definition {
                                name: Some("Living room".into()),
                                address: "A4:C1:38:E1:6F:B2".into(),
                                temperature: Range {
                                    min: Some(19.0),
                                    max: Some(22.0),
                                },
                                humidity: Range {
                                    min: Some(30.0),
                                    max: Some(60.0),
                                },
                                battery: Range {
                                    min: Some(10.0),
                                    max: None,
                                },
                            }
                            .into(),
                        ),
                        card::CardConfig::AtcSensor(
                            chezmoi_ui_static::component::card::atc_sensor::Definition {
                                name: Some("Bedroom".into()),
                                address: "A4:C1:38:45:51:3E".into(),
                                temperature: Range {
                                    min: Some(19.0),
                                    max: Some(22.0),
                                },
                                humidity: Range {
                                    min: Some(30.0),
                                    max: Some(60.0),
                                },
                                battery: Range {
                                    min: Some(10.0),
                                    max: None,
                                },
                            }
                            .into(),
                        ),
                        card::CardConfig::AtcSensor(
                            chezmoi_ui_static::component::card::atc_sensor::Definition {
                                name: Some("Office".into()),
                                address: "A4:C1:38:1C:02:76".into(),
                                temperature: Range {
                                    min: Some(19.0),
                                    max: Some(22.0),
                                },
                                humidity: Range {
                                    min: Some(30.0),
                                    max: Some(60.0),
                                },
                                battery: Range {
                                    min: Some(10.0),
                                    max: None,
                                },
                            }
                            .into(),
                        ),
                        card::CardConfig::AtcSensor(
                            chezmoi_ui_static::component::card::atc_sensor::Definition {
                                name: Some("Outside".into()),
                                address: "A4:C1:38:4E:92:06".into(),
                                temperature: Range {
                                    min: Some(5.0),
                                    max: Some(22.0),
                                },
                                humidity: Range {
                                    min: Some(30.0),
                                    max: Some(60.0),
                                },
                                battery: Range {
                                    min: Some(10.0),
                                    max: None,
                                },
                            }
                            .into(),
                        ),
                    ],
                },
                SectionConfig {
                    title: String::from("Plants"),
                    cards: [
                        "5C:85:7E:B0:4C:3F",
                        "5C:85:7E:B0:4C:9C",
                        "5C:85:7E:B0:4C:6D",
                        "C4:7C:8D:6C:D4:54",
                    ]
                    .into_iter()
                    .map(|addr| {
                        card::CardConfig::MifloraSensor(
                            chezmoi_ui_static::component::card::miflora_sensor::Definition {
                                name: None,
                                address: addr.into(),
                                temperature: Range::default(),
                                brightness: Range::default(),
                                conductivity: Range::default(),
                                moisture: Range::default(),
                                battery: Range {
                                    min: Some(10.0),
                                    max: None,
                                },
                            }
                            .into(),
                        )
                    })
                    .collect::<Vec<_>>(),
                },
            ],
        }
    }
}

impl DashboardConfig {
    pub fn collect<'a>(&'a self) -> QueryCollector<'a> {
        let mut res = QueryCollector::default();
        self.sections.iter().for_each(|s| s.collect(&mut res));
        res
    }

    pub fn build<'a>(&'a self, res: &QueryResult) -> dashboard::DashboardView<'a> {
        dashboard::DashboardView::new(self.sections.iter().map(|s| s.build(res)).collect())
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct RootConfig {
    pub home: DashboardConfig,
    pub atc_sensor: HashMap<String, DashboardConfig>,
    pub miflora_sensor: HashMap<String, DashboardConfig>,
}

fn home_dashboard(config: &crate::config::RootConfig) -> DashboardConfig {
    let mut sections = Vec::with_capacity(2);
    if !config.atc_sensor.is_empty() {
        sections.push(SectionConfig {
            title: "Thermometers".into(),
            cards: config
                .atc_sensor
                .iter()
                .map(|s| {
                    CardConfig::AtcSensor(card::atc_sensor::Config {
                        inner: s.clone(),
                        headers: card::atc_sensor::Headers::from_address(&s.address),
                    })
                })
                .collect(),
        });
    }
    if !config.miflora_sensor.is_empty() {
        sections.push(SectionConfig {
            title: "Plants".into(),
            cards: config
                .miflora_sensor
                .iter()
                .map(|s| {
                    CardConfig::MifloraSensor(card::miflora_sensor::Config {
                        inner: s.clone(),
                        headers: card::miflora_sensor::Headers::from_address(&s.address),
                    })
                })
                .collect(),
        });
    }

    DashboardConfig { sections }
}

fn atc_sensor_dashboard(
    def: &chezmoi_ui_static::component::card::atc_sensor::Definition,
) -> DashboardConfig {
    DashboardConfig {
        sections: vec![SectionConfig {
            title: "History".into(),
            cards: vec![
                card::CardConfig::History(card::history::Config {
                    definition: line_chart::Definition {
                        title: "Temperature".into(),
                        y_range: def.temperature.clone(),
                    },
                    query: MetricHeader::new("atc-thermometer.temperature")
                        .with_tag("address", def.address.clone()),
                }),
                card::CardConfig::History(card::history::Config {
                    definition: line_chart::Definition {
                        title: "Humidity".into(),
                        y_range: def.humidity.clone(),
                    },
                    query: MetricHeader::new("atc-thermometer.humidity")
                        .with_tag("address", def.address.clone()),
                }),
                card::CardConfig::History(card::history::Config {
                    definition: line_chart::Definition {
                        title: "Battery".into(),
                        y_range: def.battery.clone(),
                    },
                    query: MetricHeader::new("atc-thermometer.battery")
                        .with_tag("address", def.address.clone()),
                }),
            ],
        }],
    }
}

fn miflora_sensor_dashboard(
    def: &chezmoi_ui_static::component::card::miflora_sensor::Definition,
) -> DashboardConfig {
    DashboardConfig {
        sections: vec![SectionConfig {
            title: "History".into(),
            cards: vec![
                card::CardConfig::History(card::history::Config {
                    definition: line_chart::Definition {
                        title: "Temperature".into(),
                        y_range: def.temperature.clone(),
                    },
                    query: MetricHeader::new("miflora.temperature")
                        .with_tag("address", def.address.clone()),
                }),
                card::CardConfig::History(card::history::Config {
                    definition: line_chart::Definition {
                        title: "Brightness".into(),
                        y_range: def.brightness.clone(),
                    },
                    query: MetricHeader::new("miflora.brightness")
                        .with_tag("address", def.address.clone()),
                }),
                card::CardConfig::History(card::history::Config {
                    definition: line_chart::Definition {
                        title: "Conductivity".into(),
                        y_range: def.conductivity.clone(),
                    },
                    query: MetricHeader::new("miflora.conductivity")
                        .with_tag("address", def.address.clone()),
                }),
                card::CardConfig::History(card::history::Config {
                    definition: line_chart::Definition {
                        title: "Moisture".into(),
                        y_range: def.moisture.clone(),
                    },
                    query: MetricHeader::new("miflora.moisture")
                        .with_tag("address", def.address.clone()),
                }),
                card::CardConfig::History(card::history::Config {
                    definition: line_chart::Definition {
                        title: "Bttery".into(),
                        y_range: def.battery.clone(),
                    },
                    query: MetricHeader::new("miflora.battery")
                        .with_tag("address", def.address.clone()),
                }),
            ],
        }],
    }
}

impl From<crate::config::RootConfig> for RootConfig {
    fn from(value: crate::config::RootConfig) -> Self {
        Self {
            home: home_dashboard(&value),
            atc_sensor: value
                .atc_sensor
                .into_iter()
                .map(|c| (c.address.clone(), atc_sensor_dashboard(&c)))
                .collect(),
            miflora_sensor: value
                .miflora_sensor
                .into_iter()
                .map(|c| (c.address.clone(), miflora_sensor_dashboard(&c)))
                .collect(),
        }
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
