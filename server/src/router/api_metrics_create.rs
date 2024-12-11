use axum::http::StatusCode;
use axum::Extension;

pub async fn handle(Extension(_writer): Extension<crate::state::StorageWriter>) -> StatusCode {
    StatusCode::NO_CONTENT
}
