use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct DirectExporter<H> {
    handler: H,
    receiver: Receiver<OneOrMany<Metric>>,
}

impl<H: super::prelude::Handler> DirectExporter<H> {
    pub fn new(receiver: Receiver<OneOrMany<Metric>>, handler: H) -> Self {
        Self { handler, receiver }
    }
}

impl<H: super::prelude::Handler + Send> super::prelude::Exporter for DirectExporter<H> {
    #[tracing::instrument(name = "direct", skip_all)]
    async fn run(mut self) {
        while let Some(next) = self.receiver.recv().await {
            self.handler.handle(next).await;
        }
    }
}
