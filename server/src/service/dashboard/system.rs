use std::collections::HashSet;

use chezmoi_client::component::card::history_chart::Card as ClientHistoryChardCard;
use chezmoi_client::component::card::system_cpu::Card as ClientCpuCard;
use chezmoi_client::component::card::system_memory::Card as ClientMemoryCard;
use chezmoi_client::component::card::system_swap::Card as ClientSwapCard;
use chezmoi_client::component::card::AnyCard as ClientAnyCard;
use chezmoi_client::component::line_chart::Serie;
use chezmoi_client::Dimension;
use chezmoi_database::metrics::MetricHeader;

use super::{BuilderContext, Size};

fn find_gauge(name: &'static str, ctx: &BuilderContext) -> Option<f64> {
    let header = MetricHeader::new(name);
    ctx.latest
        .get(&header)
        .and_then(|(_, value)| value.as_gauge())
}

#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct SystemCpuCard;

impl From<SystemCpuCard> for super::AnyCard {
    fn from(value: SystemCpuCard) -> Self {
        Self::SystemCpu(value)
    }
}

impl SystemCpuCard {
    pub fn collect_latest_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        buffer.insert(MetricHeader::new(
            chezmoi_agent::sensor::system::GLOBAL_CPU_USAGE,
        ));
    }

    pub async fn build_card(&self, ctx: &BuilderContext) -> Result<ClientAnyCard, String> {
        Ok(ClientAnyCard::Cpu(ClientCpuCard::new(find_gauge(
            chezmoi_agent::sensor::system::GLOBAL_CPU_USAGE,
            ctx,
        ))))
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct SystemCpuHistoryCard {
    #[serde(default = "Size::sm")]
    height: Size,
    #[serde(default = "Size::md")]
    width: Size,
}

impl From<SystemCpuHistoryCard> for super::AnyCard {
    fn from(value: SystemCpuHistoryCard) -> Self {
        Self::SystemCpuHistory(value)
    }
}

impl SystemCpuHistoryCard {
    pub fn collect_history_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        buffer.insert(MetricHeader::new(
            chezmoi_agent::sensor::system::GLOBAL_CPU_USAGE,
        ));
    }

    pub async fn build_card(&self, ctx: &BuilderContext) -> Result<ClientAnyCard, String> {
        let header = MetricHeader::new(chezmoi_agent::sensor::system::GLOBAL_CPU_USAGE);
        let cpu_values = ctx
            .history
            .get(&header)
            .map(|list| {
                list.iter()
                    .filter_map(|(ts, value)| value.as_gauge().map(|v| (*ts, v.avg)))
                    .collect()
            })
            .unwrap_or_default();
        Ok(ClientAnyCard::HistoryChart(ClientHistoryChardCard::new(
            "CPU usage",
            Dimension::new(self.width.into(), self.height.into()),
            vec![Serie::new("CPU", cpu_values)],
            Some(ctx.window.0..ctx.window.1),
            Some(0.0..100.0),
        )))
    }
}

#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct SystemMemoryCard;

impl From<SystemMemoryCard> for super::AnyCard {
    fn from(value: SystemMemoryCard) -> Self {
        Self::SystemMemory(value)
    }
}

impl SystemMemoryCard {
    pub fn collect_latest_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        buffer.insert(MetricHeader::new(
            chezmoi_agent::sensor::system::MEMORY_USED,
        ));
        buffer.insert(MetricHeader::new(
            chezmoi_agent::sensor::system::MEMORY_TOTAL,
        ));
    }

    pub async fn build_card(&self, ctx: &BuilderContext) -> Result<ClientAnyCard, String> {
        Ok(ClientAnyCard::Memory(ClientMemoryCard::new(
            find_gauge(chezmoi_agent::sensor::system::MEMORY_TOTAL, ctx),
            find_gauge(chezmoi_agent::sensor::system::MEMORY_USED, ctx),
        )))
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct SystemMemoryHistoryCard {
    #[serde(default = "Size::sm")]
    height: Size,
    #[serde(default = "Size::md")]
    width: Size,
}

impl From<SystemMemoryHistoryCard> for super::AnyCard {
    fn from(value: SystemMemoryHistoryCard) -> Self {
        Self::SystemMemoryHistory(value)
    }
}

impl SystemMemoryHistoryCard {
    pub fn collect_history_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        buffer.insert(MetricHeader::new(
            chezmoi_agent::sensor::system::MEMORY_USED,
        ));
        buffer.insert(MetricHeader::new(
            chezmoi_agent::sensor::system::MEMORY_TOTAL,
        ));
    }

    pub async fn build_card(&self, ctx: &BuilderContext) -> Result<ClientAnyCard, String> {
        let used_header = MetricHeader::new(chezmoi_agent::sensor::system::MEMORY_USED);
        let total_header = MetricHeader::new(chezmoi_agent::sensor::system::MEMORY_TOTAL);
        let used_values: Vec<(u64, f64)> = ctx
            .history
            .get(&used_header)
            .map(|list| {
                list.iter()
                    .filter_map(|(ts, value)| value.as_gauge().map(|v| (*ts, v.avg)))
                    .collect()
            })
            .unwrap_or_default();
        let total_values: Vec<(u64, f64)> = ctx
            .history
            .get(&total_header)
            .map(|list| {
                list.iter()
                    .filter_map(|(ts, value)| value.as_gauge().map(|v| (*ts, v.avg)))
                    .collect()
            })
            .unwrap_or_default();
        let values = super::helper::merge_timelines(&used_values, &total_values)
            .map(|(ts, used, total)| (ts, used * 100.0 / total))
            .collect::<Vec<(u64, f64)>>();
        Ok(ClientAnyCard::HistoryChart(ClientHistoryChardCard::new(
            "Memory usage",
            Dimension::new(self.width.into(), self.height.into()),
            vec![Serie::new("Memory usage", values)],
            Some(ctx.window.0..ctx.window.1),
            Some(0.0..100.0),
        )))
    }
}

#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct SystemSwapCard;

impl From<SystemSwapCard> for super::AnyCard {
    fn from(value: SystemSwapCard) -> Self {
        Self::SystemSwap(value)
    }
}

impl SystemSwapCard {
    pub fn collect_latest_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        buffer.insert(MetricHeader::new(chezmoi_agent::sensor::system::SWAP_USED));
        buffer.insert(MetricHeader::new(chezmoi_agent::sensor::system::SWAP_TOTAL));
    }

    pub async fn build_card(&self, ctx: &BuilderContext) -> Result<ClientAnyCard, String> {
        Ok(ClientAnyCard::Swap(ClientSwapCard::new(
            find_gauge(chezmoi_agent::sensor::system::SWAP_TOTAL, ctx),
            find_gauge(chezmoi_agent::sensor::system::SWAP_USED, ctx),
        )))
    }
}
