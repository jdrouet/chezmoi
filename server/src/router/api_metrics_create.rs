use axum::http::StatusCode;
use axum::{Extension, Json};
use chezmoi_entity::metric::Metric;

pub async fn handle(
    Extension(writer): Extension<crate::state::StorageWriter>,
    Json(payload): Json<Vec<Metric>>,
) -> StatusCode {
    if let Err(err) = writer.sender.send(payload).await {
        tracing::error!(message = "unable to send payload to writer", error = %err);
    }
    StatusCode::NO_CONTENT
}
