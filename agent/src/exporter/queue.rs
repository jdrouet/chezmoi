use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub struct Queue {
    sender: mpsc::Sender<OneOrMany<Metric>>,
}

impl Queue {
    #[inline(always)]
    pub fn new(sender: mpsc::Sender<OneOrMany<Metric>>) -> Self {
        Self { sender }
    }
}

impl super::prelude::Handler for Queue {
    async fn handle(&mut self, events: OneOrMany<Metric>) {
        if let Err(err) = self.sender.send(events).await {
            tracing::error!(message = "unable to forward metrics", error = %err);
        }
    }
}
