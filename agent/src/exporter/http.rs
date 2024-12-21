use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug)]
enum FlushOrigin {
    Timer,
    Capacity,
}

pub const fn default_capacity() -> usize {
    50
}

pub const fn default_interval() -> u64 {
    30
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    address: String,
    #[serde(default = "default_capacity")]
    capacity: usize,
    #[serde(default = "default_interval")]
    interval: u64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Config {
            address: std::env::var("AGENT_EXPORTER_ADDRESS")
                .unwrap_or_else(|_| String::from("http://localhost:3000/api/metrics")),
            capacity: crate::from_env_or("AGENT_EXPORTER_BATCH_CAPACITY", default_capacity)?,
            interval: crate::from_env_or("AGENT_EXPORTER_BATCH_INTERVAL", default_interval)?,
        })
    }

    pub fn build(&self) -> Exporter {
        Exporter {
            address: self.address.clone(),
            client: reqwest::Client::builder()
                .user_agent(USER_AGENT)
                .build()
                .unwrap(),

            capacity: self.capacity,
            interval: self.interval,
        }
    }
}

pub struct Exporter {
    address: String,
    client: reqwest::Client,

    capacity: usize,
    interval: u64,
}

impl Exporter {
    #[tracing::instrument(name = "handle", skip(self, values))]
    async fn handle(&self, origin: FlushOrigin, values: Vec<Metric>) {
        if values.is_empty() {
            return;
        }
        match self
            .client
            .post(self.address.as_str())
            .json(&values)
            .send()
            .await
        {
            Ok(res) if res.status().is_success() => {
                tracing::info!(message = "metrics sent", count = values.len())
            }
            Ok(res) => {
                let status = res.status();
                match res.text().await {
                    Ok(payload) => {
                        tracing::error!(
                            message = "something went wrong",
                            code = status.as_u16(),
                            message = payload
                        );
                    }
                    Err(err) => {
                        tracing::error!(message = "unable to read error message", code = status.as_u16(), error = %err);
                    }
                }
            }
            Err(err) => {
                tracing::error!(message = "unable contact server", error = %err);
            }
        }
    }

    #[tracing::instrument(name = "http", skip_all)]
    pub async fn run(self, mut receiver: mpsc::Receiver<OneOrMany<Metric>>) {
        let mut flush_ticker = tokio::time::interval(std::time::Duration::new(self.interval, 0));
        let mut buffer: Vec<Metric> = Vec::with_capacity(self.capacity);

        while !receiver.is_closed() {
            tokio::select! {
                _ = flush_ticker.tick() => {
                    if !buffer.is_empty() {
                        let mut new_buffer = Vec::with_capacity(self.capacity);
                        std::mem::swap(&mut buffer, &mut new_buffer);
                        self.handle(FlushOrigin::Timer, new_buffer).await;
                    }
                },
                Some(next) = receiver.recv() => {
                    match next {
                        OneOrMany::One(value) => buffer.push(value),
                        OneOrMany::Many(values) => buffer.extend(values.into_iter()),
                    };
                    if buffer.len() >= self.capacity {
                        let mut new_buffer = Vec::with_capacity(self.capacity);
                        std::mem::swap(&mut buffer, &mut new_buffer);
                        self.handle(FlushOrigin::Capacity, new_buffer).await;
                        flush_ticker.reset();
                    }
                }
                else => break,
            }
        }
    }
}
