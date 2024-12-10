use chezmoi_entity::metric::Metric;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> From<T> for OneOrMany<T> {
    fn from(value: T) -> Self {
        Self::One(value)
    }
}

impl<T> From<Vec<T>> for OneOrMany<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Many(value)
    }
}

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
