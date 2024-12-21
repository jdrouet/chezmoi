use std::collections::HashSet;

use chezmoi_entity::metric::{Metric, MetricHeader};
use chezmoi_ui_static::component::range::Range;
use chezmoi_ui_static::view::dashboard;

mod card;

#[derive(Debug, serde::Deserialize)]
pub struct SectionConfig {
    title: String,
    #[serde(default)]
    cards: Vec<card::CardConfig>,
}

impl SectionConfig {
    fn latest_filters<'a>(&'a self, list: &mut HashSet<MetricHeader<'a>>) {
        self.cards.iter().for_each(|c| c.latest_filters(list));
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
            }],
        }
    }
}

impl DashboardConfig {
    pub fn latest_filters<'a>(&'a self) -> HashSet<MetricHeader<'a>> {
        let mut res = HashSet::new();
        self.sections
            .iter()
            .for_each(|s| s.latest_filters(&mut res));
        res
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
