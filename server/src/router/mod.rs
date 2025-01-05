use axum::routing::{get, head, post};

mod api_error;
mod api_metrics_create;
mod api_metrics_latest;
mod api_status;
mod ui_asset;
mod ui_error;
mod ui_helper;
mod ui_home;
mod ui_xiaomi_atc;
mod ui_xiaomi_miflora;

pub fn create() -> axum::Router {
    axum::Router::new()
        .route("/", get(ui_home::handle))
        .route("/xiaomi-atc/:address", get(ui_xiaomi_atc::handle))
        .route("/xiaomi-miflora/:address", get(ui_xiaomi_miflora::handle))
        .merge(ui_asset::create())
        .route("/api/metrics", post(api_metrics_create::handle))
        .route(
            "/api/metrics/latest",
            post(api_metrics_latest::handle_post).get(api_metrics_latest::handle_get),
        )
        .route("/api/status", head(api_status::handle))
}
