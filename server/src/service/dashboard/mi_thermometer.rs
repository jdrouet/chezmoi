use std::borrow::Cow;
use std::collections::HashSet;

use chezmoi_client::component::card::mi_thermometer::{Card, LastValues};
use chezmoi_client::component::card::AnyCard as ClientAnyCard;
use chezmoi_database::metrics::MetricHeader;

use super::BuilderContext;

fn header(name: &'static str, address: Cow<'static, str>) -> MetricHeader {
    MetricHeader::new(name).with_tag("address", address)
}

fn find_gauge(name: &'static str, address: Cow<'static, str>, ctx: &BuilderContext) -> Option<f64> {
    let header = header(name, address);
    ctx.latest
        .get(&header)
        .and_then(|(_, value)| value.as_gauge())
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct MiThermometerCard {
    #[serde(default)]
    name: Option<Cow<'static, str>>,
    address: Cow<'static, str>,
}

impl From<MiThermometerCard> for super::AnyCard {
    fn from(value: MiThermometerCard) -> Self {
        Self::MiThermometer(value)
    }
}

impl MiThermometerCard {
    pub fn collect_latest_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        buffer.insert(header("mithermometer.temperature", self.address.clone()));
        buffer.insert(header("mithermometer.humidity", self.address.clone()));
        buffer.insert(header("mithermometer.battery", self.address.clone()));
    }

    pub async fn build_card(&self, ctx: &BuilderContext) -> Result<ClientAnyCard, String> {
        let temperature = find_gauge("mithermometer.temperature", self.address.clone(), ctx);
        let humidity = find_gauge("mithermometer.brightness", self.address.clone(), ctx);
        let battery = find_gauge("mithermometer.battery", self.address.clone(), ctx);

        Ok(ClientAnyCard::MiThermometer(Card::new(
            self.address.as_ref(),
            self.name.as_deref(),
            LastValues {
                timestamp: None,
                temperature,
                humidity,
                battery,
            },
        )))
    }
}
