use std::borrow::Cow;
use std::collections::HashSet;

use chezmoi_agent::sensor::atc_thermometer::{DEVICE_BATTERY, DEVICE_HUMIDITY, DEVICE_TEMPERATURE};
use chezmoi_client::component::card::atc_thermometer::{Card, LastValues};
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
) -> Option<(u64, f64)> {
    let header = header(name, address);
    ctx.latest
        .get(&header)
        .and_then(|(ts, v)| v.as_gauge().map(|v| (*ts, v)))
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct AtcThermometerCard {
    #[serde(default)]
    name: Option<Cow<'static, str>>,
    address: Cow<'static, str>,
}

impl From<AtcThermometerCard> for super::AnyCard {
    fn from(value: AtcThermometerCard) -> Self {
        Self::AtcThermometer(value)
    }
}

impl AtcThermometerCard {
    pub fn collect_latest_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        buffer.insert(header(DEVICE_BATTERY, self.address.clone()));
        buffer.insert(header(DEVICE_TEMPERATURE, self.address.clone()));
        buffer.insert(header(DEVICE_HUMIDITY, self.address.clone()));
    }

    pub async fn build_card(&self, ctx: &BuilderContext) -> Result<ClientAnyCard, String> {
        let temperature = find_gauge(DEVICE_TEMPERATURE, self.address.clone(), ctx);
        let humidity = find_gauge(DEVICE_HUMIDITY, self.address.clone(), ctx);
        let battery = find_gauge(DEVICE_BATTERY, self.address.clone(), ctx);

        let timestamp = temperature
            .map(|(ts, _)| ts)
            .or(humidity.map(|(ts, _)| ts))
            .or(battery.map(|(ts, _)| ts));

        Ok(ClientAnyCard::AtcThermometer(Card::new(
            self.address.as_ref(),
            self.name.as_deref(),
            LastValues {
                timestamp,
                temperature: temperature.map(|(_, v)| v),
                humidity: humidity.map(|(_, v)| v),
                battery: battery.map(|(_, v)| v),
            },
        )))
    }
}
