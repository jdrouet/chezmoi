use axum::routing::head;

mod status;

pub(super) fn create() -> axum::Router {
    axum::Router::new().route("/status", head(status::handle))
}
