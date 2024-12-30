use chezmoi_ui_static::component::card::Card;

use crate::helper::{QueryCollector, QueryResult};

pub mod atc_sensor;
pub mod history;
pub mod miflora_sensor;

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CardConfig {
    AtcSensor(atc_sensor::Config),
    History(history::Config),
    MifloraSensor(miflora_sensor::Config),
}

impl CardConfig {
    pub fn collect<'a>(&'a self, collector: &mut QueryCollector<'a>) {
        match self {
            Self::AtcSensor(inner) => inner.collect(collector),
            Self::History(inner) => inner.collect(collector),
            Self::MifloraSensor(inner) => inner.collect(collector),
        }
    }

    pub fn build<'a>(&'a self, metrics: &QueryResult) -> Card<'a> {
        match self {
            Self::AtcSensor(inner) => Card::AtcSensor(inner.build(metrics)),
            Self::History(inner) => Card::LineChart(inner.build(metrics)),
            Self::MifloraSensor(inner) => Card::MifloraSensor(inner.build(metrics)),
        }
    }
}

// #[derive(Clone, Copy, Debug, Default, serde::Deserialize)]
// pub struct Range {
//     pub min: Option<f64>,
//     pub max: Option<f64>,
// }

// impl From<Range> for chezmoi_ui_static::component::range::Range {
//     fn from(value: Range) -> Self {
//         chezmoi_ui_static::component::range::Range {
//             min: value.min,
//             max: value.max,
//         }
//     }
// }
