use chezmoi_entity::metric::Metric;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub struct Queue {
    sender: mpsc::Sender<Vec<Metric>>,
}

impl Queue {
    #[inline(always)]
    pub fn new(sender: mpsc::Sender<Vec<Metric>>) -> Self {
        Self { sender }
    }
}

impl super::prelude::Target for Queue {
    async fn flush(&self, metrics: Vec<chezmoi_entity::metric::Metric>) -> anyhow::Result<()> {
        if let Err(err) = self.sender.send(metrics).await {
            tracing::error!(message = "unable to forward metrics", error = %err);
        }
        Ok(())
    }
}
