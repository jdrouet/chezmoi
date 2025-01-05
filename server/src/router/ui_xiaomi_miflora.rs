use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::response::Html;
use axum::Extension;

use super::ui_helper::SharedParams;
use crate::helper::{HistoryResult, LatestResult, QueryCollector, QueryResult};
use crate::router::ui_error::UiError;
use crate::view::RootConfig;

pub async fn handle(
    Extension(client): Extension<chezmoi_storage::client::Client>,
    Extension(config): Extension<Arc<RootConfig>>,
    Query(params): Query<SharedParams>,
    Path(addr): Path<String>,
) -> Result<Html<String>, UiError> {
    let Some(config) = config.xiaomi_miflora.get(&addr) else {
        return Err(UiError::not_found(
            "Provided miflora sensor address not found",
        ));
    };
    let timerange = params.timerange.as_secs();
    let QueryCollector { latest, history } = config.collect();
    let latests = if latest.is_empty() {
        Vec::new()
    } else {
        chezmoi_storage::metric::latest(client.as_ref(), latest.into_iter(), timerange).await?
    };
    let history = if history.is_empty() {
        Vec::new()
    } else {
        chezmoi_storage::metric::history(
            client.as_ref(),
            history.into_iter(),
            params.timerange.partitions(),
            timerange,
        )
        .await?
    };
    let res = QueryResult {
        timerange,
        latest: LatestResult::from_iter(latests.into_iter()),
        history: HistoryResult::from_iter(history.into_iter()),
    };
    Ok(Html(config.build(res).render()))
}
