use chezmoi_entity::metric::Metric;
use chezmoi_entity::{CowStr, OneOrMany};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub struct HttpHandler {
    address: CowStr<'static>,
    client: reqwest::Client,
}

impl HttpHandler {
    #[inline(always)]
    pub fn new(address: impl Into<CowStr<'static>>) -> Self {
        Self {
            address: address.into(),
            client: reqwest::Client::builder()
                .user_agent(USER_AGENT)
                .build()
                .unwrap(),
        }
    }
}

impl super::prelude::Handler for HttpHandler {
    #[tracing::instrument(name = "http", skip_all)]
    async fn handle(&mut self, values: OneOrMany<Metric>) {
        if values.is_empty() {
            return;
        }
        let values = values.into_vec();
        match self
            .client
            .post(self.address.as_ref())
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
}
