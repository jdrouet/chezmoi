pub struct Trace;

impl super::prelude::Target for Trace {
    async fn flush(&self, metrics: Vec<chezmoi_entity::metric::Metric>) -> anyhow::Result<()> {
        metrics.into_iter().enumerate().for_each(|(index, metric)| {
            tracing::info!(message = "received event", index = index, metric = ?metric);
        });
        Ok(())
    }
}
