use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc;

pub(crate) trait SenderExt<T> {
    async fn send_one(&self, item: T);
    #[allow(unused)]
    async fn send_many(&self, item: Vec<T>);
}

impl<T> SenderExt<T> for mpsc::Sender<OneOrMany<T>> {
    async fn send_one(&self, item: T) {
        if let Err(err) = self.send(OneOrMany::One(item)).await {
            tracing::error!(message = "unable to send events", error = %err);
        }
    }

    async fn send_many(&self, list: Vec<T>) {
        if let Err(err) = self.send(OneOrMany::Many(list)).await {
            tracing::error!(message = "unable to send events", error = %err);
        }
    }
}

pub trait Collector: Send + Sized + Sync {
    fn run(self) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
