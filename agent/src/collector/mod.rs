use chezmoi_entity::metric::Metric;
use prelude::OneOrMany;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

pub mod internal;

pub mod prelude;

#[derive(Debug)]
pub struct CollectorManager {
    context: prelude::Context,
    inner: Vec<JoinHandle<anyhow::Result<()>>>,
}

impl CollectorManager {
    pub fn new(sender: Sender<OneOrMany<Metric>>) -> Self {
        Self {
            context: prelude::Context::new(sender),
            inner: Vec::new(),
        }
    }

    pub fn start<C: prelude::Collector + 'static>(&mut self, mut collector: C) {
        let ctx = self.context.clone();
        self.inner
            .push(tokio::spawn(async move { collector.run(ctx).await }));
    }

    pub async fn wait(&mut self) {
        while let Some(job) = self.inner.pop() {
            if let Err(err) = job.await {
                tracing::error!(message = "unable to wait for job", error = %err);
            }
        }
    }
}
