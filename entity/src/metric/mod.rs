use indexmap::IndexMap;

use crate::CowStr;

#[derive(Debug)]
pub struct Metric {
    pub timestamp: u64,
    pub header: Header<'static>,
    pub value: f64,
}

#[derive(Debug)]
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
}

#[derive(Debug, Default)]
pub struct MetricTags<'a>(IndexMap<CowStr<'a>, CowStr<'a>>);

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
