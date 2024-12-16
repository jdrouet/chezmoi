use axum::response::Html;
use axum::Extension;
use chezmoi_ui_static::view::dashboard::DashboardView;

use crate::router::ui_error::UiError;

pub async fn handle(
    Extension(_client): Extension<chezmoi_storage::client::Client>,
) -> Result<Html<String>, UiError> {
    Ok(Html(DashboardView::new("/", Vec::new()).render()))
}
