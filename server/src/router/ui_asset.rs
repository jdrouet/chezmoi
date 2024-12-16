use axum::http::header::CONTENT_TYPE;
use axum::response::{AppendHeaders, IntoResponse};

pub async fn style_css() -> impl IntoResponse {
    (
        AppendHeaders([(CONTENT_TYPE, "text/css")]),
        chezmoi_ui_static::asset::STYLE_CSS,
    )
}
