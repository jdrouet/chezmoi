use chezmoi_database::metrics::entity::Metric;
use tokio::sync::mpsc;

pub mod sensor;

pub const HOSTNAME: &str = "hostname";
pub const ADDRESS: &str = "address";

pub struct Config {
    #[cfg(feature = "bluetooth")]
    bluetooth: sensor::bluetooth::Config,
    system: sensor::system::Config,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            #[cfg(feature = "bluetooth")]
            bluetooth: sensor::bluetooth::Config::from_env()?,
            system: sensor::system::Config::from_env()?,
        })
    }

    pub async fn build(self) -> anyhow::Result<Agent> {
        Ok(Agent {
            #[cfg(feature = "bluetooth")]
            bluetooth: self.bluetooth.build().await?,
            system: self.system.build()?,
        })
    }
}

#[derive(Debug)]
pub struct Agent {
    #[cfg(feature = "bluetooth")]
    bluetooth: Option<sensor::bluetooth::Sensor>,
    system: Option<sensor::system::Sensor>,
}

impl Agent {
    pub async fn run(self, database: chezmoi_database::Client) -> anyhow::Result<()> {
        let (sender, mut receiver) = mpsc::channel::<Vec<Metric>>(100);

        let context = sensor::Context::new(true, sender);

        let mut sensors = Vec::new();
        #[cfg(feature = "bluetooth")]
        if let Some(bluetooth) = self.bluetooth {
            let ctx = context.clone();
            sensors.push(tokio::spawn(async move { bluetooth.run(ctx).await }));
        }
        if let Some(system) = self.system {
            let ctx = context.clone();
            sensors.push(tokio::spawn(async move { system.run(ctx).await }));
        }

        while let Some(batch) = receiver.recv().await {
            tracing::debug!(message = "received events", count = batch.len());
            if batch.is_empty() {
                continue;
            }
            match chezmoi_database::metrics::entity::create::Command::new(&batch)
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
