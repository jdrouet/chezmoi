use axum::{Extension, Json};
use chezmoi_entity::metric::{Metric, MetricHeader};
use serde_qs::axum::QsQuery;

use crate::router::api_error::ApiError;

#[derive(Debug, serde::Deserialize)]
pub struct Payload {
    headers: Vec<MetricHeader<'static>>,
    from: u64,
    to: u64,
}

pub async fn handle_post(
    Extension(client): Extension<chezmoi_storage::client::Client>,
    Json(payload): Json<Payload>,
) -> Result<Json<Vec<Metric>>, ApiError> {
    if payload.headers.is_empty() {
        return Ok(Json(Vec::new()));
    }
    let list = chezmoi_storage::metric::latest(
        client.as_ref(),
        payload.headers.iter(),
        (payload.from, payload.to),
    )
    .await?;
    Ok(Json(list))
}

pub async fn handle_get(
    Extension(client): Extension<chezmoi_storage::client::Client>,
    QsQuery(payload): QsQuery<Payload>,
) -> Result<Json<Vec<Metric>>, ApiError> {
    if payload.headers.is_empty() {
        return Ok(Json(Vec::new()));
    }
    let list = chezmoi_storage::metric::latest(
        client.as_ref(),
        payload.headers.iter(),
        (payload.from, payload.to),
    )
    .await?;
    Ok(Json(list))
}
