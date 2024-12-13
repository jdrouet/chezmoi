use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chezmoi_storage::sqlx;

#[derive(Debug, serde::Serialize)]
pub(super) struct ApiError {
    #[serde(skip)]
    code: StatusCode,
    message: &'static str,
}

impl ApiError {
    pub fn internal() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "something went wrong",
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        tracing::error!(message = "something went wrong with the database", error = %value);
        Self::internal()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.code, Json(self)).into_response()
    }
}
