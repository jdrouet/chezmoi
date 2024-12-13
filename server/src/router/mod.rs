use axum::routing::{head, post};

mod api_error;
mod api_metrics_create;
mod api_metrics_latest;
mod api_status;

pub fn create() -> axum::Router {
    axum::Router::new()
        .route("/api/metrics", post(api_metrics_create::handle))
        .route(
            "/api/metrics/latest",
            post(api_metrics_latest::handle_post).get(api_metrics_latest::handle_get),
        )
        .route("/api/status", head(api_status::handle))
}
