use axum::response::Html;
use axum::Extension;
use chezmoi_client::component::any_card::AnyCard;
#[cfg(feature = "bluetooth")]
use chezmoi_client::component::bluetooth_devices_card::BluetoothDevicesCard;
use chezmoi_client::component::cpu_card::CpuCard;
use chezmoi_client::component::memory_card::MemoryCard;
use chezmoi_client::component::swap_card::SwapCard;
use chezmoi_client::view::home::Section;
use chezmoi_client::view::prelude::View;
use chezmoi_database::metrics::entity::Metric;

use super::error::Error;

#[cfg(feature = "bluetooth")]
fn collect_gauges_with_tag<'a>(
    metrics: &'a [Metric],
    metric_name: &'static str,
    tag: &'static str,
) -> impl Iterator<Item = (&'a std::borrow::Cow<'static, str>, f64)> + 'a {
    metrics
        .iter()
        .filter(|metric| metric.header.name.as_ref().eq(metric_name))
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

fn collect_latest_gauge(metrics: &[Metric], metric_name: &'static str) -> Option<f64> {
    metrics
        .iter()
        .filter(|metric| metric.header.name.as_ref().eq(metric_name))
        .filter_map(|item| item.value.as_gauge().map(|value| (item.timestamp, value)))
        .max_by(|first, second| first.0.cmp(&second.0))
        .map(|(_, value)| value)
}

#[cfg(feature = "bluetooth")]
fn extract_bluetooth_devices_card(metrics: &mut Vec<Metric>) -> BluetoothDevicesCard {
    BluetoothDevicesCard::new(collect_gauges_with_tag(
        metrics,
        "bluetooth.device.power",
        "address",
    ))
}

fn extract_memory_card(metrics: &[Metric]) -> Option<MemoryCard> {
    let total = collect_latest_gauge(metrics, chezmoi_agent::sensor::system::MEMORY_TOTAL);
    let used = collect_latest_gauge(metrics, chezmoi_agent::sensor::system::MEMORY_USED);
    match (total, used) {
        (Some(total), Some(used)) => Some(MemoryCard::new(total, used)),
        _ => None,
    }
}

fn extract_swap_card(metrics: &[Metric]) -> Option<SwapCard> {
    let total = collect_latest_gauge(metrics, chezmoi_agent::sensor::system::SWAP_TOTAL);
    let used = collect_latest_gauge(metrics, chezmoi_agent::sensor::system::SWAP_USED);
    match (total, used) {
        (Some(total), Some(used)) => Some(SwapCard::new(total, used)),
        _ => None,
    }
}

fn extract_cpu_card(metrics: &mut Vec<Metric>) -> Option<CpuCard> {
    collect_latest_gauge(metrics, chezmoi_agent::sensor::system::GLOBAL_CPU_USAGE).map(CpuCard::new)
}

pub(super) async fn handle(
    Extension(database): Extension<chezmoi_database::Client>,
) -> Result<Html<String>, Error> {
    let mut headers = Vec::new();
    #[cfg(feature = "bluetooth")]
    headers.push(chezmoi_database::metrics::MetricHeader::new(
        chezmoi_agent::sensor::bluetooth::DEVICE_POWER,
    ));
    headers.push(chezmoi_database::metrics::MetricHeader::new(
        chezmoi_agent::sensor::system::GLOBAL_CPU_USAGE,
    ));
    headers.push(chezmoi_database::metrics::MetricHeader::new(
        chezmoi_agent::sensor::system::MEMORY_USED,
    ));
    headers.push(chezmoi_database::metrics::MetricHeader::new(
        chezmoi_agent::sensor::system::MEMORY_TOTAL,
    ));
    headers.push(chezmoi_database::metrics::MetricHeader::new(
        chezmoi_agent::sensor::system::SWAP_USED,
    ));
    headers.push(chezmoi_database::metrics::MetricHeader::new(
        chezmoi_agent::sensor::system::SWAP_TOTAL,
    ));
    let mut latests = chezmoi_database::metrics::entity::find_latest::Command::new(&headers, None)
        .execute(database.as_ref())
        .await?;

    let page = chezmoi_client::view::home::View::new();

    #[cfg(feature = "bluetooth")]
    let page = {
        let bluetooth_devices = extract_bluetooth_devices_card(&mut latests);
        let scanner =
            Section::new("Scanner").with_card(AnyCard::BluetoothDevices(bluetooth_devices));
        page.with_section(scanner)
    };

    let page = {
        let memory = extract_memory_card(&mut latests).map(AnyCard::Memory);
        let swap = extract_swap_card(&mut latests).map(AnyCard::Swap);
        let cpu = extract_cpu_card(&mut latests).map(AnyCard::Cpu);
        let system = Section::new("System")
            .maybe_with_card(memory)
            .maybe_with_card(swap)
            .maybe_with_card(cpu);
        page.with_section(system)
    };

    Ok(Html(page.render()))
}
