use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

pub(super) fn router() -> axum::Router {
    let path = std::env::var("ASSETS_PATH").ok().unwrap_or_default();
    axum::Router::new()
        .nest_service("/assets", ServeDir::new(path))
        .layer(CompressionLayer::new())
}
