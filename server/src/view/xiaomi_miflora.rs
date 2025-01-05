use std::collections::HashSet;

use another_html_builder::Buffer;
use chezmoi_web_prelude::prelude::RenderComponent;

use crate::helper::{QueryCollector, QueryResult};

#[derive(Clone, Debug)]
pub struct Config(pub chezmoi_sensor_xiaomi_miflora::web::Config);

impl Config {
    pub fn collect(&self) -> QueryCollector {
        QueryCollector {
            history: HashSet::from_iter(self.0.headers.list()),
            latest: HashSet::from_iter(self.0.headers.list()),
        }
    }

    pub fn build<'a>(&'a self, res: QueryResult) -> Component<'a> {
        Component {
            config: &self.0,
            values: res,
        }
    }
}

pub struct Component<'a> {
    config: &'a chezmoi_sensor_xiaomi_miflora::web::Config,
    values: QueryResult,
}

impl<'a> Component<'a> {
    pub fn render(&self) -> String {
        let buf = Buffer::default();
        buf.render_component(chezmoi_sensor_xiaomi_miflora::web::board::Component {
            definition: &self.config.definition,
            timerange: self.values.timerange,
            values: chezmoi_sensor_xiaomi_miflora::web::board::Values {
                temperature: self
                    .values
                    .history
                    .find(&self.config.headers.temperature)
                    .unwrap_or_default(),
                brightness: self
                    .values
                    .history
                    .find(&self.config.headers.brightness)
                    .unwrap_or_default(),
                conductivity: self
                    .values
                    .history
                    .find(&self.config.headers.conductivity)
                    .unwrap_or_default(),
                moisture: self
                    .values
                    .history
                    .find(&self.config.headers.moisture)
                    .unwrap_or_default(),
                battery: self
                    .values
                    .history
                    .find(&self.config.headers.battery)
                    .unwrap_or_default(),
            },
        })
        .into_inner()
    }
}
