use chezmoi_entity::metric::Metric;
use chezmoi_ui_static::component::card::Card;

pub mod atc_sensor;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CardConfig {
    AtcSensor(atc_sensor::Config),
}

impl CardConfig {
    pub fn build<'a>(&'a self, metrics: &[Metric]) -> Card<'a> {
        match self {
            Self::AtcSensor(inner) => Card::AtcSensor(inner.build(metrics)),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, serde::Deserialize)]
pub struct Range {
    min: Option<f64>,
    max: Option<f64>,
}

impl From<Range> for chezmoi_ui_static::component::range::Range {
    fn from(value: Range) -> Self {
        chezmoi_ui_static::component::range::Range {
            min: value.min,
            max: value.max,
        }
    }
}
