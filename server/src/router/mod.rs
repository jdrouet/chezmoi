mod api;
mod asset;
mod ui;

pub(super) fn create() -> axum::Router {
    axum::Router::new()
        .nest("/api", api::create())
        .merge(asset::router())
        .merge(ui::create())
}
