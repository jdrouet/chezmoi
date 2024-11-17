use axum::http::StatusCode;
use axum::Extension;

pub(crate) async fn handle(Extension(database): Extension<chezmoi_database::Client>) -> StatusCode {
    match database.ping().await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(inner) => {
            tracing::error!(message = "unable to ping database", cause = %inner);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
