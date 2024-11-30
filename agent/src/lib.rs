use chezmoi_database::metrics::entity::Metric;
use tokio::sync::mpsc;

pub mod sensor;

pub const HOSTNAME: &str = "hostname";
pub const ADDRESS: &str = "address";

#[cfg(feature = "bluetooth")]
async fn default_bt_adapter() -> anyhow::Result<bluer::Adapter> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    Ok(adapter)
}

pub struct Config {
    #[cfg(feature = "sensor-bt-scanner")]
    bt_scanner: sensor::bt_scanner::Config,
    #[cfg(feature = "sensor-miflora")]
    miflora: sensor::miflora::Config,
    system: sensor::system::Config,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            #[cfg(feature = "sensor-bt-scanner")]
            bt_scanner: sensor::bt_scanner::Config::from_env()?,
            #[cfg(feature = "sensor-miflora")]
            miflora: sensor::miflora::Config::from_env()?,
            system: sensor::system::Config::from_env()?,
        })
    }

    pub async fn build(self) -> anyhow::Result<Agent> {
        #[cfg(feature = "bluetooth")]
        let bt_adapter = default_bt_adapter().await?;

        Ok(Agent {
            #[cfg(feature = "sensor-bt-scanner")]
            bt_scanner: self.bt_scanner.build(bt_adapter).await?,
            #[cfg(feature = "sensor-miflora")]
            miflora: self.miflora.build(bt_adapter).await?,
            system: self.system.build()?,
        })
    }
}

#[derive(Debug)]
pub struct Agent {
    #[cfg(feature = "sensor-bt-scanner")]
    bt_scanner: Option<sensor::bt_scanner::Sensor>,
    #[cfg(feature = "sensor-miflora")]
    miflora: Option<sensor::miflora::Sensor>,
    system: Option<sensor::system::Sensor>,
}

impl Agent {
    pub async fn run(self, database: chezmoi_database::Client) -> anyhow::Result<()> {
        let (sender, mut receiver) = mpsc::channel::<Vec<Metric>>(100);

        let context = sensor::Context::new(true, sender);

        let mut sensors = Vec::new();
        #[cfg(feature = "sensor-bt-scanner")]
        if let Some(sensor) = self.bt_scanner {
            let ctx = context.clone();
            sensors.push(tokio::spawn(async move { sensor.run(ctx).await }));
        }
        #[cfg(feature = "sensor-miflora")]
        if let Some(sensor) = self.miflora {
            let ctx = context.clone();
            sensors.push(tokio::spawn(async move { sensor.run(ctx).await }));
        }
        if let Some(sensor) = self.system {
            let ctx = context.clone();
            sensors.push(tokio::spawn(async move { sensor.run(ctx).await }));
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
