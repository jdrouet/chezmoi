use std::{collections::HashMap, sync::Arc};

mod home;
mod xiaomi_atc;
mod xiaomi_miflora;

#[derive(Clone, Debug)]
pub struct RootConfig {
    pub home: home::Config,
    pub xiaomi_atc: HashMap<String, xiaomi_atc::Config>,
    pub xiaomi_miflora: HashMap<String, xiaomi_miflora::Config>,
}

impl RootConfig {
    pub fn new(value: Arc<crate::config::RootConfig>) -> Self {
        Self {
            home: home::Config(value.clone()),
            xiaomi_atc: HashMap::from_iter(
                value
                    .xiaomi_atc
                    .iter()
                    .map(|c| (c.definition.address.clone(), xiaomi_atc::Config(c.clone()))),
            ),
            xiaomi_miflora: HashMap::from_iter(value.xiaomi_miflora.iter().map(|c| {
                (
                    c.definition.address.clone(),
                    xiaomi_miflora::Config(c.clone()),
                )
            })),
        }
    }
}
