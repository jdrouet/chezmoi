use std::io::BufReader;
use std::path::Path;

use anyhow::Context;
use chezmoi_ui_static::component::card::{atc_sensor, miflora_sensor};

#[derive(Debug, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RootConfig {
    #[serde(default)]
    pub atc_sensor: Vec<atc_sensor::Definition>,
    #[serde(default)]
    pub miflora_sensor: Vec<miflora_sensor::Definition>,
}

impl RootConfig {
    pub fn from_path<P: AsRef<Path>>(path: &P) -> anyhow::Result<Self> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .open(path)
            .context("unable to open config file")?;
        let reader = BufReader::new(f);
        serde_json::from_reader(reader).context("unable to parse json content")
    }
}
