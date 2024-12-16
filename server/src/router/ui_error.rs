use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chezmoi_storage::sqlx;

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
}

impl From<sqlx::Error> for UiError {
    fn from(value: sqlx::Error) -> Self {
        tracing::error!(message = "something went wrong with the database", error = %value);
        Self::internal()
    }
}

impl IntoResponse for UiError {
    fn into_response(self) -> axum::response::Response {
        (self.code, Json(self)).into_response()
    }
}
