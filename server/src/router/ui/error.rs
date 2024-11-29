use std::borrow::Cow;

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use chezmoi_client::view::prelude::View;

#[derive(Debug)]
pub struct Error {
    status: StatusCode,
    message: Cow<'static, str>,
}

impl Error {
    pub fn new(status: StatusCode, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl From<chezmoi_database::sqlx::Error> for Error {
    fn from(value: chezmoi_database::sqlx::Error) -> Self {
        tracing::error!(message = "something went wrong with database", cause = %value);
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let html = chezmoi_client::view::error::View::new(self.message).render();
        (self.status, Html(html)).into_response()
    }
}
