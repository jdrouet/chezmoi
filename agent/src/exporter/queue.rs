use chezmoi_entity::metric::Metric;
use tokio::sync::mpsc;

use super::batch::BatchHandler;
use crate::collector::prelude::OneOrMany;

#[derive(Clone, Debug)]
pub struct Queue {
    sender: mpsc::Sender<Vec<Metric>>,
}

impl Queue {
    #[inline(always)]
    pub fn new(sender: mpsc::Sender<Vec<Metric>>) -> Self {
        Self { sender }
    }

    async fn forward(&self, metrics: Vec<Metric>) {
        if let Err(err) = self.sender.send(metrics).await {
            tracing::error!(message = "unable to forward metrics", error = %err);
        }
    }
}

impl super::prelude::Exporter for Queue {
    async fn run(&self, mut receiver: mpsc::Receiver<OneOrMany<Metric>>) {
        while let Some(batch) = receiver.recv().await {
            match batch {
                OneOrMany::One(item) => self.forward(vec![item]).await,
                OneOrMany::Many(items) => self.forward(items).await,
            }
        }
    }
}

pub struct BatchQueueHandler {
    sender: mpsc::Sender<Vec<Metric>>,
}

impl BatchQueueHandler {
    #[inline(always)]
    pub fn new(sender: mpsc::Sender<Vec<Metric>>) -> Self {
        Self { sender }
    }
}

impl BatchHandler for BatchQueueHandler {
    async fn handle(&self, values: Vec<Metric>) {
        if let Err(err) = self.sender.send(values).await {
            tracing::error!(message = "unable to forward metrics", error = %err);
        }
    }
}
