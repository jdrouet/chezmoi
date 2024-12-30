use chezmoi_entity::metric::MetricHeader;
use chezmoi_ui_static::component::card::line_chart;

use crate::helper::{QueryCollector, QueryResult};

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub definition: line_chart::Definition,
    pub query: MetricHeader<'static>,
}

impl Config {
    pub fn collect<'a>(&'a self, res: &mut QueryCollector<'a>) {
        res.history.insert(&self.query);
    }

    pub fn build<'a>(&'a self, res: &QueryResult) -> line_chart::LineChartCard<'a> {
        line_chart::LineChartCard {
            definition: &self.definition,
            values: res.history.find(&self.query).unwrap_or_default(),
        }
    }
}
