#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct Range<V> {
    #[serde(default)]
    pub min: Option<V>,
    #[serde(default)]
    pub max: Option<V>,
}

impl<V> From<(V, V)> for Range<V> {
    fn from((min, max): (V, V)) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
        }
    }
}
