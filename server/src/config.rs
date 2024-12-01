use std::path::Path;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct RootConfig {
    #[serde(default)]
    pub agent: chezmoi_agent::Config,
    #[serde(default)]
    pub database: chezmoi_database::Config,
    #[serde(default)]
    pub server: crate::app::Config,
}

impl RootConfig {
    pub fn from_path(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::de::from_str(content.as_str())?)
    }
}
