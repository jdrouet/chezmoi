use chezmoi_database::metrics::entity::Metric;
use tokio::sync::mpsc;

mod sensor;

pub struct Config {
    system: sensor::system::Config,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            system: sensor::system::Config::from_env()?,
        })
    }

    pub async fn build(self) -> anyhow::Result<Agent> {
        Ok(Agent {
            system: self.system.build()?,
        })
    }
}

#[derive(Debug)]
pub struct Agent {
    system: Option<sensor::system::Sensor>,
}

impl Agent {
    pub async fn run(self, database: chezmoi_database::Client) -> anyhow::Result<()> {
        let Self { system } = self;
        let (sender, mut receiver) = mpsc::channel::<Vec<Metric>>(100);

        let context = sensor::Context::new(true, sender);

        let mut sensors = Vec::new();
        if let Some(system) = system {
            sensors.push(tokio::spawn(async move { system.run(context).await }));
        }

        while let Some(batch) = receiver.recv().await {
            tracing::debug!(message = "received events", count = batch.len());
            match chezmoi_database::metrics::Create::new(&batch)
                .execute(database.as_ref())
                .await
            {
                Ok(count) => tracing::debug!(message = "stored events", count = count),
                Err(error) => {
                    tracing::error!(message = "unable to store received metrics", cause = %error)
                }
            }
        }

        while let Some(sensor) = sensors.pop() {
            match sensor.await {
                Ok(Ok(_)) => {}
                Ok(Err(inner)) => tracing::error!(message = "sensor failed", cause = %inner),
                Err(inner) => tracing::error!(message = "unable to join sensor", cause = %inner),
            }
        }

        Ok(())
    }
}
