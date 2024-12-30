use std::sync::Arc;

use axum::extract::Query;
use axum::response::Html;
use axum::Extension;
use chezmoi_ui_static::view::prelude::View;

use super::ui_helper::SharedParams;
use crate::entity::RootConfig;
use crate::helper::{HistoryResult, LatestResult, QueryCollector, QueryResult};
use crate::router::ui_error::UiError;

pub async fn handle(
    Extension(client): Extension<chezmoi_storage::client::Client>,
    Extension(config): Extension<Arc<RootConfig>>,
    Query(params): Query<SharedParams>,
) -> Result<Html<String>, UiError> {
    let timerange = params.timerange.as_secs();
    let QueryCollector { latest, history } = config.home.collect();
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
        latest: LatestResult::from_iter(latests.into_iter()),
        history: HistoryResult::from_iter(history.into_iter()),
    };
    Ok(Html(config.home.build(&res).render(&super::UI_CTX)))
}
