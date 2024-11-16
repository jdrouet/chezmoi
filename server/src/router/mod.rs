mod api;

pub(super) fn create() -> axum::Router {
    axum::Router::new().nest("/api", api::create())
}
