use std::borrow::Cow;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MetricTagValue {
    Text(Cow<'static, str>),
    Float(f64),
    Int(i64),
    Boolean(bool),
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MetricTags(pub indexmap::IndexMap<String, MetricTagValue>);

impl MetricTags {
    #[inline]
    pub fn set(&mut self, name: String, value: MetricTagValue) -> Option<MetricTagValue> {
        self.0.insert(name, value)
    }

    pub fn with(mut self, name: String, value: MetricTagValue) -> Self {
        self.0.insert(name, value);
        self
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl MetricTags {
    pub fn urlencode(&self) -> String {
        serde_urlencoded::to_string(&self.0).expect("serialize a map")
    }

    pub fn urldecode(input: &str) -> Self {
        serde_urlencoded::from_str(input).expect("deserialize a map")
    }
}
