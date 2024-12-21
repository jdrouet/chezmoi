#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct Range {
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
}
