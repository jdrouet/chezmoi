use axum::routing::get;

mod error;
mod home;

pub(super) fn create() -> axum::Router {
    axum::Router::new().route("/", get(home::handle))
}
