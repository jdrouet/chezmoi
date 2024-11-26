use axum::response::Html;
use axum::Extension;
use chezmoi_client::component::any_card::AnyCard;
use chezmoi_client::component::bluetooth_devices_card::BluetoothDevicesCard;
use chezmoi_client::view::home::Section;
use chezmoi_client::view::prelude::View;
use chezmoi_database::metrics::MetricHeader;

use super::error::Error;

pub(super) async fn handle(
    Extension(database): Extension<chezmoi_database::Client>,
) -> Result<Html<String>, Error> {
    let headers = vec![MetricHeader::new("bluetooth.device.power")];
    let mut latests = chezmoi_database::metrics::entity::find_latest::Command::new(&headers, None)
        .execute(database.as_ref())
        .await?;

    let bluetooth_devices = BluetoothDevicesCard::new(
        latests
            .extract_if(|metric| metric.header.name.as_ref().eq("bluetooth.device.power"))
            .filter_map(|item| {
                match (
                    item.header
                        .tags
                        .extract("address")
                        .and_then(|v| v.into_text()),
                    item.value.as_gauge(),
                ) {
                    (Some(address), Some(value)) => Some((address, value)),
                    _ => None,
                }
            }),
    );

    Ok(Html(
        chezmoi_client::view::home::View::new(chezmoi_client::asset::STYLE_CSS_PATH)
            .with_section(
                Section::new("Scanner").with_card(AnyCard::BluetoothDevices(bluetooth_devices)),
            )
            .render(),
    ))
}
