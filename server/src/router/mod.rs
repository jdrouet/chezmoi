mod api;
mod ui;

pub(super) fn create() -> axum::Router {
    axum::Router::new()
        .nest("/api", api::create())
        .merge(ui::create())
}
