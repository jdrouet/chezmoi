use std::future::Future;

use chezmoi_entity::metric::Metric;
use tokio::sync::mpsc::Receiver;

use crate::collector::prelude::OneOrMany;

pub trait DirectHandler {
    fn handle(&mut self, values: OneOrMany<Metric>) -> impl Future<Output = ()> + Send;
}

#[derive(Debug)]
pub struct DirectExporter<H> {
    handler: H,
}

impl<H: DirectHandler> DirectExporter<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
}

impl<H: DirectHandler + Send> super::prelude::Exporter for DirectExporter<H> {
    #[tracing::instrument(name = "collector", skip_all)]
    async fn run(mut self, mut receiver: Receiver<OneOrMany<Metric>>) {
        while let Some(next) = receiver.recv().await {
            self.handler.handle(next).await;
        }
    }
}
