#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MetricName(pub String);

impl MetricName {
    pub fn from_parts(namespace: &str, name: &str) -> Self {
        Self(format!("{namespace}.{name}"))
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
        self.0.as_str()
    }
}

impl std::fmt::Display for MetricName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
