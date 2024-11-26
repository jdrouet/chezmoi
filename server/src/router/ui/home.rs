use std::borrow::Cow;

use axum::response::Html;
use axum::Extension;
use chezmoi_client::component::any_card::AnyCard;
use chezmoi_client::component::bluetooth_devices_card::BluetoothDevicesCard;
use chezmoi_client::view::home::Section;
use chezmoi_client::view::prelude::View;
use chezmoi_database::metrics::entity::Metric;
use chezmoi_database::metrics::MetricHeader;

use super::error::Error;

fn collect_gauges_with_tag<'a>(
    metrics: &'a mut Vec<Metric>,
    metric_name: &'static str,
    tag: &'static str,
) -> impl Iterator<Item = (Cow<'static, str>, f64)> + 'a {
    metrics
        .extract_if(|metric| metric.header.name.as_ref().eq(metric_name))
        .filter_map(|item| {
            match (
                item.header.tags.extract(tag).and_then(|v| v.into_text()),
                item.value.as_gauge(),
            ) {
                (Some(address), Some(value)) => Some((address, value)),
                _ => None,
            }
        })
}

fn extract_bluetooth_devices_card(metrics: &mut Vec<Metric>) -> BluetoothDevicesCard {
    BluetoothDevicesCard::new(collect_gauges_with_tag(
        metrics,
        "bluetooth.device.power",
        "address",
    ))
}

pub(super) async fn handle(
    Extension(database): Extension<chezmoi_database::Client>,
) -> Result<Html<String>, Error> {
    let headers = vec![MetricHeader::new("bluetooth.device.power")];
    let mut latests = chezmoi_database::metrics::entity::find_latest::Command::new(&headers, None)
        .execute(database.as_ref())
        .await?;

    let bluetooth_devices = extract_bluetooth_devices_card(&mut latests);

    let scanner = Section::new("Scanner").with_card(AnyCard::BluetoothDevices(bluetooth_devices));

    Ok(Html(
        chezmoi_client::view::home::View::new(chezmoi_client::asset::STYLE_CSS_PATH)
            .with_section(scanner)
            .render(),
    ))
}
