use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub mod aggr;
pub mod entity;
pub mod macros;

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct MetricHeader {
    pub name: MetricName,
    #[serde(skip_serializing_if = "MetricTags::is_empty")]
    pub tags: MetricTags,
}

impl MetricHeader {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: MetricName::new(name),
            tags: Default::default(),
        }
    }

    pub fn with_tag<N: Into<Cow<'static, str>>, V: Into<MetricTagValue>>(
        mut self,
        name: N,
        value: V,
    ) -> Self {
        self.tags.set(name, value);
        self
    }

    pub fn into_hash(&self) -> u64 {
        let mut s = std::hash::DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MetricName(pub Cow<'static, str>);

impl MetricName {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self(name.into())
    }
}

impl AsRef<str> for MetricName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::fmt::Display for MetricName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MetricTagValue {
    Text(Cow<'static, str>),
    ArcText(Arc<String>),
    Float(f64),
    Int(i64),
    Boolean(bool),
}

impl MetricTagValue {
    pub fn into_text(self) -> Option<Cow<'static, str>> {
        match self {
            Self::Text(inner) => Some(inner),
            _ => None,
        }
    }
}

impl From<&'static str> for MetricTagValue {
    fn from(value: &'static str) -> Self {
        Self::Text(Cow::Borrowed(value))
    }
}

impl From<String> for MetricTagValue {
    fn from(value: String) -> Self {
        Self::Text(Cow::Owned(value))
    }
}

impl From<f64> for MetricTagValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for MetricTagValue {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<bool> for MetricTagValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl std::hash::Hash for MetricTagValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Text(inner) => inner.hash(state),
            Self::ArcText(inner) => inner.hash(state),
            Self::Float(inner) => inner.to_bits().hash(state),
            Self::Int(inner) => inner.hash(state),
            Self::Boolean(inner) => inner.hash(state),
        }
    }
}

impl Eq for MetricTagValue {}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MetricTags(pub indexmap::IndexMap<Cow<'static, str>, MetricTagValue>);

impl std::hash::Hash for MetricTags {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.iter().for_each(|(key, value)| {
            key.hash(state);
            value.hash(state);
        });
    }
}

impl MetricTags {
    pub fn remove(&mut self, name: &str) -> Option<MetricTagValue> {
        self.0.shift_remove(name)
    }

    pub fn extract(mut self, name: &str) -> Option<MetricTagValue> {
        self.0.swap_remove(name)
    }

    #[inline]
    pub fn set<N: Into<Cow<'static, str>>, V: Into<MetricTagValue>>(
        &mut self,
        name: N,
        value: V,
    ) -> Option<MetricTagValue> {
        self.0.insert(name.into(), value.into())
    }

    pub fn with<N: Into<Cow<'static, str>>, V: Into<MetricTagValue>>(
        mut self,
        name: N,
        value: V,
    ) -> Self {
        self.0.insert(name.into(), value.into());
        self
    }

    pub fn maybe_with<N: Into<Cow<'static, str>>, V: Into<MetricTagValue>>(
        mut self,
        name: N,
        value: Option<V>,
    ) -> Self {
        if let Some(value) = value {
            self.0.insert(name.into(), value.into());
        }
        self
    }

    pub fn entries(&self) -> impl Iterator<Item = (&Cow<'static, str>, &MetricTagValue)> {
        self.0.iter()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
