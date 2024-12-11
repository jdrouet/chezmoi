use std::future::Future;

use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;

pub trait Exporter {
    fn run(self) -> impl Future<Output = ()> + Send;
}

pub trait Handler {
    fn handle(&mut self, events: OneOrMany<Metric>) -> impl Future<Output = ()> + Send;
}
