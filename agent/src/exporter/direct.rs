use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct DirectExporter<H> {
    handler: H,
}

impl<H: super::prelude::Handler> DirectExporter<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
}

impl<H: super::prelude::Handler + Send> super::prelude::Exporter for DirectExporter<H> {
    #[tracing::instrument(name = "direct", skip_all)]
    async fn run(mut self, mut receiver: Receiver<OneOrMany<Metric>>) {
        while let Some(next) = receiver.recv().await {
            self.handler.handle(next).await;
        }
    }
}
