use std::sync::Arc;

use axum::response::Html;
use axum::Extension;
use chezmoi_entity::now;
use chezmoi_ui_static::view::prelude::View;

use crate::entity::DashboardConfig;
use crate::helper::{HistoryResult, LatestResult, QueryCollector, QueryResult};
use crate::router::ui_error::UiError;

pub async fn handle(
    Extension(client): Extension<chezmoi_storage::client::Client>,
    Extension(config): Extension<Arc<DashboardConfig>>,
) -> Result<Html<String>, UiError> {
    let ts = now();
    let QueryCollector { latest, history: _ } = config.collect();
    let latests = if latest.is_empty() {
        Vec::new()
    } else {
        chezmoi_storage::metric::latest(client.as_ref(), latest.into_iter(), (0, ts)).await?
    };
    let res = QueryResult {
        latest: LatestResult::from_iter(latests.into_iter()),
        history: HistoryResult::from_iter(std::iter::empty()),
    };
    Ok(Html(config.build(&res).render(&super::UI_CTX)))
}
