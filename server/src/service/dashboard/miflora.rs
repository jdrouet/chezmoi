use std::borrow::Cow;
use std::collections::HashSet;

use chezmoi_client::component::card::miflora::{Card, TimedValue, Values};
use chezmoi_client::component::card::AnyCard as ClientAnyCard;
use chezmoi_database::metrics::MetricHeader;

use super::BuilderContext;

fn header(name: &'static str, address: Cow<'static, str>) -> MetricHeader {
    MetricHeader::new(name).with_tag("address", address)
}

fn find_gauge(
    name: &'static str,
    address: Cow<'static, str>,
    ctx: &BuilderContext,
) -> Option<TimedValue> {
    let header = header(name, address);
    ctx.latest
        .get(&header)
        .and_then(|(ts, value)| value.as_gauge().map(|v| (*ts, v)))
        .map(TimedValue::from)
}

#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct Range {
    min: Option<f64>,
    max: Option<f64>,
}

impl Range {
    fn as_tuple(&self) -> (Option<f64>, Option<f64>) {
        (self.min, self.max)
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
            Values {
                temperature: find_gauge("miflora.temperature", self.address.clone(), ctx),
                temperature_range: self.temperature.as_tuple(),
                brightness: find_gauge("miflora.brightness", self.address.clone(), ctx),
                brightness_range: self.brightness.as_tuple(),
                moisture: find_gauge("miflora.moisture", self.address.clone(), ctx),
                moisture_range: self.moisture.as_tuple(),
                conductivity: find_gauge("miflora.conductivity", self.address.clone(), ctx),
                conductivity_range: self.conductivity.as_tuple(),
                battery: find_gauge("miflora.battery", self.address.clone(), ctx),
                battery_range: self.battery.as_tuple(),
            },
        )))
    }
}
