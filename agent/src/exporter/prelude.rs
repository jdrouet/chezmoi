use std::future::Future;

use chezmoi_entity::metric::Metric;

pub trait Target {
    fn flush(&self, metrics: Vec<Metric>) -> impl Future<Output = anyhow::Result<()>> + Send;
}
