use std::collections::HashSet;

use chezmoi_client::component::card::system_cpu::Card as ClientCpuCard;
use chezmoi_client::component::card::system_memory::Card as ClientMemoryCard;
use chezmoi_client::component::card::system_swap::Card as ClientSwapCard;
use chezmoi_client::component::card::AnyCard as ClientAnyCard;
use chezmoi_database::metrics::MetricHeader;

use super::BuilderContext;

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
