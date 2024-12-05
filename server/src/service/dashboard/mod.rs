use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use chezmoi_client::component::card::AnyCard as ClientAnyCard;
use chezmoi_client::view::dashboard::{self, TimePickerDuration};
use chezmoi_database::metrics::entity::{Metric, MetricValue};
use chezmoi_database::metrics::MetricHeader;

#[cfg(feature = "bluetooth")]
pub(crate) mod atc_thermometer;
#[cfg(feature = "bluetooth")]
pub(crate) mod miflora;
pub(crate) mod system;

mod helper;

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Size {
    Sm,
    Md,
}

impl Size {
    pub fn sm() -> Self {
        Self::Sm
    }

    pub fn md() -> Self {
        Self::Md
    }
}

impl From<Size> for chezmoi_client::Size {
    fn from(value: Size) -> Self {
        match value {
            Size::Sm => chezmoi_client::Size::Sm,
            Size::Md => chezmoi_client::Size::Md,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub(crate) enum AnyCard {
    #[cfg(feature = "bluetooth")]
    AtcThermometer(atc_thermometer::AtcThermometerCard),
    #[cfg(feature = "bluetooth")]
    Miflora(miflora::MifloraCard),
    SystemCpu(system::SystemCpuCard),
    SystemCpuHistory(system::SystemCpuHistoryCard),
    SystemMemory(system::SystemMemoryCard),
    SystemMemoryHistory(system::SystemMemoryHistoryCard),
    SystemSwap(system::SystemSwapCard),
}

impl AnyCard {
    pub fn collect_latest_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        match self {
            #[cfg(feature = "bluetooth")]
            Self::AtcThermometer(inner) => inner.collect_latest_metrics(buffer),
            #[cfg(feature = "bluetooth")]
            Self::Miflora(inner) => inner.collect_latest_metrics(buffer),
            Self::SystemCpu(inner) => inner.collect_latest_metrics(buffer),
            Self::SystemMemory(inner) => inner.collect_latest_metrics(buffer),
            Self::SystemSwap(inner) => inner.collect_latest_metrics(buffer),
            _ => {}
        }
    }

    pub fn collect_history_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        match self {
            Self::SystemCpuHistory(inner) => inner.collect_history_metrics(buffer),
            Self::SystemMemoryHistory(inner) => inner.collect_history_metrics(buffer),
            _ => {}
        }
    }

    pub async fn build_card(&self, ctx: &BuilderContext) -> Result<ClientAnyCard, String> {
        match self {
            #[cfg(feature = "bluetooth")]
            Self::AtcThermometer(inner) => inner.build_card(ctx).await,
            #[cfg(feature = "bluetooth")]
            Self::Miflora(inner) => inner.build_card(ctx).await,
            Self::SystemCpu(inner) => inner.build_card(ctx).await,
            Self::SystemCpuHistory(inner) => inner.build_card(ctx).await,
            Self::SystemMemory(inner) => inner.build_card(ctx).await,
            Self::SystemMemoryHistory(inner) => inner.build_card(ctx).await,
            Self::SystemSwap(inner) => inner.build_card(ctx).await,
        }
    }
}

#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct Section {
    name: Cow<'static, str>,
    #[serde(default)]
    cards: Vec<AnyCard>,
}

impl Section {
    pub fn collect_latest_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        self.cards
            .iter()
            .for_each(|card| card.collect_latest_metrics(buffer));
    }

    pub fn collect_history_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        self.cards
            .iter()
            .for_each(|card| card.collect_history_metrics(buffer));
    }
}

#[derive(Debug)]
pub struct BuilderContext {
    window: (u64, u64),
    duration: TimePickerDuration,
    latest: HashMap<MetricHeader, (u64, MetricValue)>,
    history: HashMap<MetricHeader, Vec<(u64, MetricValue)>>,
}

impl BuilderContext {
    pub fn new(duration: TimePickerDuration, window: (u64, u64)) -> Self {
        Self {
            window,
            duration,
            latest: Default::default(),
            history: Default::default(),
        }
    }

    pub fn add_latests(&mut self, list: impl Iterator<Item = Metric>) {
        self.latest
            .extend(list.map(|metric| (metric.header, (metric.timestamp, metric.value))));
    }

    pub fn add_history(&mut self, list: impl Iterator<Item = Metric>) {
        list.for_each(|metric| {
            let entry = self.history.entry(metric.header).or_default();
            entry.push((metric.timestamp, metric.value));
        });
    }
}

#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct Dashboard {
    #[serde(default)]
    sections: Vec<Section>,
}

impl Dashboard {
    pub fn collect_latest_metrics(&self) -> Vec<MetricHeader> {
        let mut buf = HashSet::new();
        self.sections
            .iter()
            .for_each(|sec| sec.collect_latest_metrics(&mut buf));
        Vec::from_iter(buf)
    }

    pub fn collect_history_metrics(&self) -> Vec<MetricHeader> {
        let mut buf = HashSet::new();
        self.sections
            .iter()
            .for_each(|sec| sec.collect_history_metrics(&mut buf));
        Vec::from_iter(buf)
    }

    pub async fn build_view(&self, ctx: BuilderContext) -> Result<dashboard::View, String> {
        let mut sections = Vec::with_capacity(self.sections.len());
        for section in self.sections.iter() {
            let mut vsec = dashboard::Section::new(section.name.as_ref());
            for card in section.cards.iter() {
                vsec.add_card(card.build_card(&ctx).await?);
            }
            sections.push(vsec);
        }
        Ok(dashboard::View::new(sections, ctx.duration))
    }
}
