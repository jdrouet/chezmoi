use chezmoi_entity::metric::Metric;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct Writer {
    #[allow(dead_code)]
    client: crate::client::Client,
}

impl Writer {
    pub fn new(client: crate::client::Client) -> Self {
        Self { client }
    }
}

impl Writer {
    #[tracing::instrument(name = "writer", skip_all)]
    pub async fn run(&self, mut receiver: Receiver<Vec<Metric>>) {
        while let Some(metrics) = receiver.recv().await {
            self.handle_metrics(metrics).await;
        }
    }

    async fn handle_metrics(&self, metrics: Vec<Metric>) {
        tracing::trace!(message = "received metrics", count = metrics.len());
        if metrics.is_empty() {
            return;
        }
        match crate::metric::create(self.client.as_ref(), metrics.iter()).await {
            Ok(count) => {
                tracing::debug!(message = "persisted metrics", count = count);
            }
            Err(err) => {
                tracing::error!(message = "unable to persist metrics", error = %err);
            }
        }
    }
}
