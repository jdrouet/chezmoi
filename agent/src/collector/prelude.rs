use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub struct Context {
    sender: mpsc::Sender<OneOrMany<Metric>>,
}

impl Context {
    pub fn new(sender: mpsc::Sender<OneOrMany<Metric>>) -> Self {
        Self { sender }
    }

    #[inline(always)]
    pub fn is_closing(&self) -> bool {
        self.sender.is_closed()
    }

    pub fn queue_size(&self) -> usize {
        self.sender.max_capacity() - self.sender.capacity()
    }

    pub async fn send<T>(&self, value: T)
    where
        T: Into<OneOrMany<Metric>>,
    {
        if let Err(err) = self.sender.send(value.into()).await {
            tracing::error!(message = "unable to forward metrics", error = %err);
        }
    }
}

pub trait Collector: Send + Sized + Sync {
    fn run(&mut self, ctx: Context)
        -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
