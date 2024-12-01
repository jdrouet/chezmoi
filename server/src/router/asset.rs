use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

pub(super) fn router(assets_path: &str) -> axum::Router {
    axum::Router::new()
        .nest_service("/assets", ServeDir::new(assets_path))
        .layer(CompressionLayer::new())
}
