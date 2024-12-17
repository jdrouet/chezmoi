use std::sync::Arc;

use axum::response::Html;
use axum::Extension;
use chezmoi_entity::now;

use crate::entity::DashboardConfig;
use crate::router::ui_error::UiError;

pub async fn handle(
    Extension(client): Extension<chezmoi_storage::client::Client>,
    Extension(config): Extension<Arc<DashboardConfig>>,
) -> Result<Html<String>, UiError> {
    let ts = now();
    let latest_headers = config.latest_filters();
    let latest_headers = Vec::from_iter(latest_headers.into_iter());
    let latests =
        chezmoi_storage::metric::latest(client.as_ref(), &latest_headers, (0, ts)).await?;
    Ok(Html(config.build(&latests).render()))
}
