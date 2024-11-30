use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use chezmoi_client::component::card::AnyCard as ClientAnyCard;
use chezmoi_client::view::dashboard;
use chezmoi_database::metrics::entity::{Metric, MetricValue};
use chezmoi_database::metrics::MetricHeader;

pub(crate) mod mi_thermometer;
pub(crate) mod miflora;
pub(crate) mod system;

#[derive(Debug)]
pub(crate) enum AnyCard {
    MiThermometer(mi_thermometer::MiThermometerCard),
    Miflora(miflora::MifloraCard),
    SystemCpu(system::SystemCpuCard),
    SystemMemory(system::SystemMemoryCard),
    SystemSwap(system::SystemSwapCard),
}

impl AnyCard {
    pub fn collect_latest_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        match self {
            Self::MiThermometer(inner) => inner.collect_latest_metrics(buffer),
            Self::Miflora(inner) => inner.collect_latest_metrics(buffer),
            Self::SystemCpu(inner) => inner.collect_latest_metrics(buffer),
            Self::SystemMemory(inner) => inner.collect_latest_metrics(buffer),
            Self::SystemSwap(inner) => inner.collect_latest_metrics(buffer),
        }
    }

    pub async fn build_card(&self, ctx: &BuilderContext) -> Result<ClientAnyCard, String> {
        match self {
            Self::MiThermometer(inner) => inner.build_card(ctx).await,
            Self::Miflora(inner) => inner.build_card(ctx).await,
            Self::SystemCpu(inner) => inner.build_card(ctx).await,
            Self::SystemMemory(inner) => inner.build_card(ctx).await,
            Self::SystemSwap(inner) => inner.build_card(ctx).await,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Section {
    name: Cow<'static, str>,
    cards: Vec<AnyCard>,
}

impl Section {
    pub fn new<N: Into<Cow<'static, str>>>(name: N) -> Self {
        Self {
            name: name.into(),
            cards: Vec::default(),
        }
    }

    pub fn collect_latest_metrics(&self, buffer: &mut HashSet<MetricHeader>) {
        self.cards
            .iter()
            .for_each(|card| card.collect_latest_metrics(buffer));
    }

    pub fn with_card(mut self, card: impl Into<AnyCard>) -> Self {
        self.cards.push(card.into());
        self
    }
}

#[derive(Debug, Default)]
pub struct BuilderContext {
    latest: HashMap<MetricHeader, (u64, MetricValue)>,
}

impl BuilderContext {
    pub fn add_latests(&mut self, list: impl Iterator<Item = Metric>) {
        self.latest
            .extend(list.map(|metric| (metric.header, (metric.timestamp, metric.value))));
    }
}

#[derive(Debug, Default)]
pub(crate) struct Dashboard {
    sections: Vec<Section>,
}

impl Dashboard {
    pub fn with_section(mut self, section: Section) -> Self {
        self.sections.push(section);
        self
    }

    pub fn collect_latest_metrics(&self) -> Vec<MetricHeader> {
        let mut buf = HashSet::new();
        self.sections
            .iter()
            .for_each(|sec| sec.collect_latest_metrics(&mut buf));
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
        Ok(dashboard::View::new(sections))
    }
}
