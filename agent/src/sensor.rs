use chezmoi_sensor_prelude::agent::prelude::SensorSender;
use chezmoi_sensor_prelude::agent::BuildContext;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Config {
    System(chezmoi_sensor_system::agent::Config),
}

impl chezmoi_sensor_prelude::agent::prelude::Config for Config {
    type Output = Sensor;

    async fn build(&self, ctx: &BuildContext) -> anyhow::Result<Self::Output> {
        match self {
            Self::System(inner) => inner.build(ctx).await.map(Sensor::System),
        }
    }
}

pub enum Sensor {
    System(chezmoi_sensor_system::agent::Sensor),
}

impl chezmoi_sensor_prelude::agent::prelude::Sensor for Sensor {
    async fn run(self, sender: SensorSender) -> anyhow::Result<()> {
        match self {
            Self::System(inner) => inner.run(sender).await,
        }
    }
}
