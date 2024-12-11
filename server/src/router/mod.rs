use axum::routing::{head, post};

mod api_metrics_create;
mod api_status;

pub fn create() -> axum::Router {
    axum::Router::new()
        .route("/api/metrics", post(api_metrics_create::handle))
        .route("/api/status", head(api_status::handle))
}
