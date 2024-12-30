use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use chezmoi_storage::sqlx;
use chezmoi_ui_static::view::prelude::View;

#[derive(Debug, serde::Serialize)]
pub(super) struct UiError {
    #[serde(skip)]
    code: StatusCode,
    message: &'static str,
}

impl UiError {
    pub fn internal() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "something went wrong",
        }
    }

    pub fn not_found(message: &'static str) -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message,
        }
    }
}

impl From<sqlx::Error> for UiError {
    fn from(value: sqlx::Error) -> Self {
        tracing::error!(message = "something went wrong with the database", error = %value);
        Self::internal()
    }
}

impl IntoResponse for UiError {
    fn into_response(self) -> axum::response::Response {
        let view = chezmoi_ui_static::view::error::ErrorView::new(self.message);
        let view = view.render(&super::UI_CTX);
        (self.code, Html(view)).into_response()
    }
}
