use axum::routing::get;
use tower_http::compression::CompressionLayer;

mod error;
mod home;

pub(super) fn create() -> axum::Router {
    axum::Router::new()
        .route("/", get(home::handle))
        .layer(CompressionLayer::new())
}
