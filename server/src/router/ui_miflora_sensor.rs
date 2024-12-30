use axum::extract::Path;
use axum::response::Html;
use axum::Extension;
use chezmoi_entity::metric::MetricHeader;
use chezmoi_entity::now;
use chezmoi_ui_static::component::range::Range;
use chezmoi_ui_static::view::prelude::View;

use crate::helper::{HistoryResult, LatestResult, QueryCollector, QueryResult};
use crate::router::ui_error::UiError;

fn create_config(addr: &str, from: u64, to: u64) -> crate::entity::DashboardConfig {
    crate::entity::DashboardConfig {
        sections: vec![crate::entity::SectionConfig {
            title: "History".into(),
            cards: [
                ("Temperature", "miflora.temperature", 0.0, 35.0),
                ("Brightness", "miflora.brightness", 0.0, 5000.0),
                ("Conductivity", "miflora.conductivity", 0.0, 1.0),
                ("Moisture", "miflora.moisture", 0.0, 100.0),
                ("Battery", "miflora.battery", 0.0, 100.0),
            ]
            .into_iter()
            .map(|(title, name, min, max)| {
                crate::entity::card::CardConfig::History(crate::entity::card::history::Config {
                    definition: chezmoi_ui_static::component::card::line_chart::Definition {
                        title: title.to_string(),
                        x_range: Range {
                            min: Some(from),
                            max: Some(to),
                        },
                        y_range: Range {
                            min: Some(min),
                            max: Some(max),
                        },
                    },
                    query: MetricHeader::new(name).with_tag("address", addr.to_string()),
                })
            })
            .collect(),
        }],
    }
}

pub async fn handle(
    Extension(client): Extension<chezmoi_storage::client::Client>,
    Path(addr): Path<String>,
) -> Result<Html<String>, UiError> {
    let ts = now();
    let from = ts - 60 * 60 * 24;
    let config = create_config(&addr, from, ts);
    let QueryCollector { latest, history } = config.collect();
    let latests = if latest.is_empty() {
        Vec::new()
    } else {
        chezmoi_storage::metric::latest(client.as_ref(), latest.into_iter(), (from, ts)).await?
    };
    let history = if history.is_empty() {
        Vec::new()
    } else {
        chezmoi_storage::metric::history(client.as_ref(), history.into_iter(), 48, (from, ts))
            .await?
    };
    let res = QueryResult {
        latest: LatestResult::from_iter(latests.into_iter()),
        history: HistoryResult::from_iter(history.into_iter()),
    };
    Ok(Html(config.build(&res).render(&super::UI_CTX)))
}
