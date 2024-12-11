use crate::BuildContext;

pub mod internal;

pub mod prelude;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Config {
    Internal(internal::Config),
}

impl Config {
    pub fn build(&self, ctx: &BuildContext) -> Collector {
        match self {
            Self::Internal(inner) => Collector::Internal(inner.build(ctx)),
        }
    }
}

pub enum Collector {
    Internal(internal::Collector),
}

impl crate::prelude::Worker for Collector {
    async fn run(self) -> anyhow::Result<()> {
        match self {
            Self::Internal(inner) => inner.run().await,
        }
    }
}
