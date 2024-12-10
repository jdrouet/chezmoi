use std::future::Future;

use chezmoi_entity::metric::Metric;
use tokio::sync::mpsc::Receiver;

use crate::collector::prelude::OneOrMany;

pub trait Exporter {
    fn run(&self, receiver: Receiver<OneOrMany<Metric>>) -> impl Future<Output = ()> + Send;
}
