use chezmoi_entity::metric::Metric;
use chezmoi_ui_static::view::dashboard;

mod card;

#[derive(Debug, serde::Deserialize)]
pub struct SectionConfig {
    title: String,
    #[serde(default)]
    cards: Vec<card::CardConfig>,
}

impl SectionConfig {
    pub fn build<'a>(&'a self, _metrics: &[Metric]) -> dashboard::Section<'a> {
        dashboard::Section::new(self.title.as_str(), Vec::new())
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct DashboardConfig {
    #[serde(default)]
    sections: Vec<SectionConfig>,
}

impl DashboardConfig {
    pub fn build<'a>(&self, _metrics: &[Metric]) -> dashboard::DashboardView<'a> {
        dashboard::DashboardView::new("/", Vec::new())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_deserialize() {
        let _: super::DashboardConfig = serde_json::from_value(serde_json::json!({
            "sections": [
                {
                    "title": "foo",
                    "cards": [
                        {
                            "type": "atc-sensor",
                            "address": "00:00:00:00:00",
                            "temperature": {
                                "min": 12.34,
                                "max": 23.45
                            }
                        }
                    ],
                }
            ]
        }))
        .unwrap();
    }
}
