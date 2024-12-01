mod api;
mod asset;
mod ui;

pub(super) fn create(assets_path: &str) -> axum::Router {
    axum::Router::new()
        .nest("/api", api::create())
        .merge(asset::router(assets_path))
        .merge(ui::create())
}
