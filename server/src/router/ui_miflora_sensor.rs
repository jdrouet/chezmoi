use std::sync::Arc;

use axum::extract::Path;
use axum::response::Html;
use axum::Extension;
use chezmoi_entity::now;
use chezmoi_ui_static::view::prelude::View;

use crate::entity::RootConfig;
use crate::helper::{HistoryResult, LatestResult, QueryCollector, QueryResult};
use crate::router::ui_error::UiError;

pub async fn handle(
    Extension(client): Extension<chezmoi_storage::client::Client>,
    Extension(config): Extension<Arc<RootConfig>>,
    Path(addr): Path<String>,
) -> Result<Html<String>, UiError> {
    let ts = now();
    let from = ts - 60 * 60 * 24;
    let Some(config) = config.miflora_sensor.get(&addr) else {
        return Err(UiError::not_found(
            "Provided miflora sensor address not found",
        ));
    };
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
