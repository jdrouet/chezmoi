use std::time::SystemTime;

#[inline]
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("current time before unix epoch")
        .as_secs()
}

#[cfg(test)]
pub(crate) async fn create_metrics(
    db: &crate::Client,
    header: crate::metrics::MetricHeader,
    values: impl Iterator<Item = (u64, crate::metrics::entity::MetricValue)>,
) -> Vec<crate::metrics::entity::Metric> {
    let metrics = values
        .map(|(timestamp, value)| crate::metrics::entity::Metric {
            timestamp,
            header: header.clone(),
            value,
        })
        .collect::<Vec<_>>();

    crate::metrics::entity::create::Command::new(&metrics)
        .execute(db.as_ref())
        .await
        .unwrap();

    metrics
}
