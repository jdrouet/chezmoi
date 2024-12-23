use std::sync::Arc;

use axum::response::Html;
use axum::Extension;
use chezmoi_entity::now;
use chezmoi_ui_static::view::prelude::View;

use crate::entity::DashboardConfig;
use crate::helper::LatestResult;
use crate::router::ui_error::UiError;

pub async fn handle(
    Extension(client): Extension<chezmoi_storage::client::Client>,
    Extension(config): Extension<Arc<DashboardConfig>>,
) -> Result<Html<String>, UiError> {
    let ts = now();
    let latest_headers = config.latest_filters();
    let latests = if latest_headers.is_empty() {
        Vec::new()
    } else {
        chezmoi_storage::metric::latest(client.as_ref(), latest_headers.into_iter(), (0, ts))
            .await?
    };
    let latests = LatestResult::from_iter(latests.into_iter());
    Ok(Html(config.build(&latests).render(&super::UI_CTX)))
}
