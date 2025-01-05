use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use chezmoi_storage::sqlx;
use chezmoi_web_prelude::component::{head, header, html};
use chezmoi_web_prelude::prelude::RenderComponent;

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
        let buf = html::html(another_html_builder::Buffer::default(), |buf| {
            buf.render_component(head::Component::new("Error", &[]))
                .node("body")
                .content(|buf| {
                    buf.render_component(header::Component::new("Error"))
                        .node("main")
                        .attr(("class", "container pad-md"))
                        .content(|buf| {
                            buf.node("div").attr(("class", "card")).content(|buf| {
                                buf.node("div")
                                    .attr(("class", "text-center pad-lg"))
                                    .content(|buf| buf.text(self.message))
                            })
                        })
                })
        });
        (self.code, Html(buf.into_inner())).into_response()
    }
}
