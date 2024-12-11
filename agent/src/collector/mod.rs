pub mod internal;

pub mod prelude;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Config {
    Internal(internal::Config),
}

impl Config {
    pub fn build(&self) -> Collector {
        match self {
            Self::Internal(inner) => Collector::Internal(inner.build()),
        }
    }
}

pub enum Collector {
    Internal(internal::Collector),
}

impl prelude::Collector for Collector {
    async fn run(self, ctx: prelude::Context) -> anyhow::Result<()> {
        match self {
            Self::Internal(inner) => inner.run(ctx).await,
        }
    }
}
