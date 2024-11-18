use std::borrow::Cow;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MetricName(pub Cow<'static, str>);

impl MetricName {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self(name.into())
    }

    pub fn from_parts(namespace: &str, name: &str) -> Self {
        Self(Cow::Owned(format!("{namespace}.{name}")))
    }

    pub fn namespace(&self) -> Option<&str> {
        self.0.rsplit_once('.').map(|(left, _)| left)
    }

    pub fn name(&self) -> Option<&str> {
        self.0.rsplit_once('.').map(|(_, right)| right)
    }

    pub fn parts(&self) -> Option<(&str, &str)> {
        self.0.rsplit_once('.')
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
