use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use indexmap::IndexMap;

use crate::CowStr;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Metric {
    pub timestamp: u64,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub header: MetricHeader<'static>,
    pub value: f64,
}

impl Metric {
    #[inline(always)]
    pub const fn new(timestamp: u64, header: MetricHeader<'static>, value: f64) -> Self {
        Self {
            timestamp,
            header,
            value,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetricHeader<'a> {
    pub name: CowStr<'a>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "MetricTags::is_empty")
    )]
    pub tags: MetricTags<'a>,
}

impl<'a> MetricHeader<'a> {
    pub fn new<N>(name: N) -> Self
    where
        N: Into<CowStr<'a>>,
    {
        Self {
            name: name.into(),
            tags: MetricTags::default(),
        }
    }

    pub fn with_tag<N, V>(mut self, name: N, value: V) -> Self
    where
        N: Into<CowStr<'a>>,
        V: Into<CowStr<'a>>,
    {
        self.tags.set(name, value);
        self
    }

    pub fn set_tag<N, V>(&mut self, name: N, value: V) -> &mut Self
    where
        N: Into<CowStr<'a>>,
        V: Into<CowStr<'a>>,
    {
        self.tags.set(name, value);
        self
    }

    pub fn into_hash(&self) -> u64 {
        let mut h = std::hash::DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct MetricTags<'a>(IndexMap<CowStr<'a>, CowStr<'a>>);

impl<'a> AsRef<IndexMap<CowStr<'a>, CowStr<'a>>> for MetricTags<'a> {
    fn as_ref(&self) -> &IndexMap<CowStr<'a>, CowStr<'a>> {
        &self.0
    }
}

impl std::fmt::Debug for MetricTags<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::hash::Hash for MetricTags<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.iter().for_each(|(name, value)| {
            name.hash(state);
            value.hash(state);
        });
    }
}

impl<'a> MetricTags<'a> {
    pub fn with<N, V>(mut self, name: N, value: V) -> Self
    where
        N: Into<CowStr<'a>>,
        V: Into<CowStr<'a>>,
    {
        self.set(name, value);
        self
    }

    pub fn set<N, V>(&mut self, name: N, value: V) -> &mut Self
    where
        N: Into<CowStr<'a>>,
        V: Into<CowStr<'a>>,
    {
        self.0.insert(name.into(), value.into());
        self
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
