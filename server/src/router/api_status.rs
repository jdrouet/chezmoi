use axum::http::StatusCode;
use axum::Extension;
use chezmoi_storage::client::Client;

pub async fn handle(Extension(reader): Extension<Client>) -> StatusCode {
    match reader.ping().await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            tracing::error!(message = "unable to ping database", error = %err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
