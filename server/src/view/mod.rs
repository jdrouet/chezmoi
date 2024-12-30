use std::collections::HashMap;

use chezmoi_ui_static::component::range::Range;
use chezmoi_ui_static::view::dashboard;

use crate::helper::{QueryCollector, QueryResult};

pub mod card;

#[derive(Debug, serde::Deserialize)]
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

#[derive(Debug, serde::Deserialize)]
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
        dashboard::DashboardView::new("/", self.sections.iter().map(|s| s.build(res)).collect())
    }
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct RootConfig {
    pub home: DashboardConfig,
    pub atc_sensor: HashMap<String, DashboardConfig>,
    pub miflora_sensor: HashMap<String, DashboardConfig>,
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
