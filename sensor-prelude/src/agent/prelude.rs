use std::future::Future;

use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc::Sender;

use crate::agent::AgentMetric;

pub trait Config {
    type Output;

    fn build(
        &self,
        ctx: &super::BuildContext,
    ) -> impl Future<Output = anyhow::Result<Self::Output>> + Send;
}

pub type SensorSender = Sender<OneOrMany<AgentMetric>>;

pub trait Sensor: Sized {
    fn run(self, sender: SensorSender) -> impl Future<Output = anyhow::Result<()>> + Send;
}
