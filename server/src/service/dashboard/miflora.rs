use std::borrow::Cow;
use std::collections::HashSet;

use chezmoi_client::component::card::miflora::{Card, LastValues, TimedValue, ValueState};
use chezmoi_client::component::card::AnyCard as ClientAnyCard;
use chezmoi_database::metrics::MetricHeader;

use super::BuilderContext;

fn header(name: &'static str, address: Cow<'static, str>) -> MetricHeader {
    MetricHeader::new(name).with_tag("address", address)
}

fn find_gauge(
    name: &'static str,
    address: Cow<'static, str>,
    expected: &Range,
    ctx: &BuilderContext,
) -> Option<TimedValue> {
    let header = header(name, address);
    ctx.latest
        .get(&header)
        .and_then(|(ts, value)| value.as_gauge().map(|v| (*ts, v, expected.evaluate(v))))
        .map(TimedValue::from)
}

#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct Range {
    min: Option<f64>,
    max: Option<f64>,
}

impl Range {
    fn evaluate(&self, value: f64) -> ValueState {
        match (self.min, self.max) {
            (Some(min), _) if value < min => ValueState::Low { min },
            (_, Some(max)) if value > max => ValueState::High { max },
            _ => ValueState::Normal,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct MifloraCard {
    #[serde(default)]
    name: Option<Cow<'static, str>>,
    address: Cow<'static, str>,
    image: Option<String>,
    #[serde(default)]
    temperature: Range,
    #[serde(default)]
    brightness: Range,
    #[serde(default)]
    moisture: Range,
    #[serde(default)]
    conductivity: Range,
    #[serde(default)]
    battery: Range,
}

impl From<MifloraCard> for super::AnyCard {
    fn from(value: MifloraCard) -> Self {
        Self::Miflora(value)
    }
}

impl MifloraCard {
    pub fn collect_latest_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        buffer.insert(header("miflora.temperature", self.address.clone()));
        buffer.insert(header("miflora.brightness", self.address.clone()));
        buffer.insert(header("miflora.moisture", self.address.clone()));
        buffer.insert(header("miflora.conductivity", self.address.clone()));
        buffer.insert(header("miflora.battery", self.address.clone()));
    }

    pub async fn build_card(&self, ctx: &BuilderContext) -> Result<ClientAnyCard, String> {
        Ok(ClientAnyCard::Miflora(Card::new(
            self.address.as_ref(),
            self.name.as_deref(),
            self.image.as_deref(),
            LastValues {
                temperature: find_gauge(
                    "miflora.temperature",
                    self.address.clone(),
                    &self.temperature,
                    ctx,
                ),
                brightness: find_gauge(
                    "miflora.brightness",
                    self.address.clone(),
                    &self.brightness,
                    ctx,
                ),
                moisture: find_gauge(
                    "miflora.moisture",
                    self.address.clone(),
                    &self.moisture,
                    ctx,
                ),
                conductivity: find_gauge(
                    "miflora.conductivity",
                    self.address.clone(),
                    &self.conductivity,
                    ctx,
                ),
                battery: find_gauge("miflora.battery", self.address.clone(), &self.battery, ctx),
            },
        )))
    }
}
