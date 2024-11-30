use std::borrow::Cow;
use std::collections::HashSet;

use chezmoi_client::component::card::miflora::{Card, LastValues, TimedValue};
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

#[derive(Debug)]
pub(crate) struct MifloraCard {
    name: Option<Cow<'static, str>>,
    address: Cow<'static, str>,
}

impl From<MifloraCard> for super::AnyCard {
    fn from(value: MifloraCard) -> Self {
        Self::Miflora(value)
    }
}

impl MifloraCard {
    pub fn new<N: Into<Cow<'static, str>>, A: Into<Cow<'static, str>>>(
        name: Option<N>,
        address: A,
    ) -> Self {
        Self {
            name: name.map(|n| n.into()),
            address: address.into(),
        }
    }

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
            LastValues {
                temperature: find_gauge("miflora.temperature", self.address.clone(), ctx),
                brightness: find_gauge("miflora.brightness", self.address.clone(), ctx),
                moisture: find_gauge("miflora.moisture", self.address.clone(), ctx),
                conductivity: find_gauge("miflora.conductivity", self.address.clone(), ctx),
                battery: find_gauge("miflora.battery", self.address.clone(), ctx),
            },
        )))
    }
}
