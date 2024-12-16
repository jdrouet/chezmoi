use axum::routing::{get, head, post};

mod api_error;
mod api_metrics_create;
mod api_metrics_latest;
mod api_status;
mod ui_asset;
mod ui_dashboard;
mod ui_error;

pub fn create() -> axum::Router {
    axum::Router::new()
        .route("/", get(ui_dashboard::handle))
        .route("/assets/style.css", get(ui_asset::style_css))
        .route("/api/metrics", post(api_metrics_create::handle))
        .route(
            "/api/metrics/latest",
            post(api_metrics_latest::handle_post).get(api_metrics_latest::handle_get),
        )
        .route("/api/status", head(api_status::handle))
}
