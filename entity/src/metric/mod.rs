use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use indexmap::IndexMap;

use crate::CowStr;

#[derive(Debug)]
pub struct Metric {
    pub timestamp: u64,
    pub header: Header<'static>,
    pub value: f64,
}

impl Metric {
    #[inline(always)]
    pub const fn new(timestamp: u64, header: Header<'static>, value: f64) -> Self {
        Self {
            timestamp,
            header,
            value,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Header<'a> {
    pub name: CowStr<'a>,
    pub tags: MetricTags<'a>,
}

impl<'a> Header<'a> {
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

#[derive(Default, PartialEq, Eq)]
pub struct MetricTags<'a>(IndexMap<CowStr<'a>, CowStr<'a>>);

impl<'a> std::fmt::Debug for MetricTags<'a> {
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
}
