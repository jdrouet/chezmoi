use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use anyhow::Context;

#[derive(Debug, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RootConfig {
    #[serde(default)]
    pub xiaomi_atc: Vec<chezmoi_sensor_xiaomi_atc::web::Config>,
    #[serde(default)]
    pub xiaomi_miflora: Vec<chezmoi_sensor_xiaomi_miflora::web::Config>,
}

impl RootConfig {
    pub fn from_path<P: AsRef<Path>>(path: &P) -> anyhow::Result<Arc<Self>> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .open(path)
            .context("unable to open config file")?;
        let reader = BufReader::new(f);
        serde_json::from_reader(reader).context("unable to parse json content")
    }
}
