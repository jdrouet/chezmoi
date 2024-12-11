use std::future::Future;

use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc::Receiver;

pub trait Exporter {
    fn run(self, receiver: Receiver<OneOrMany<Metric>>) -> impl Future<Output = ()> + Send;
}

pub trait Handler {
    fn handle(&mut self, events: OneOrMany<Metric>) -> impl Future<Output = ()> + Send;
}
