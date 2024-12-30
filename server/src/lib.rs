use std::str::FromStr;
use std::sync::Arc;

use anyhow::Context;
use axum::Extension;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod config;
mod helper;
mod router;
mod state;
mod view;

fn from_env_or<T, F>(name: &str, default_value: F) -> anyhow::Result<T>
where
    F: FnOnce() -> T,
    T: FromStr,
    anyhow::Error: From<<T as FromStr>::Err>,
{
    if let Ok(value) = std::env::var(name) {
        Ok(T::from_str(value.as_str())?)
    } else {
        Ok(default_value())
    }
}

#[derive(Debug)]
pub struct Config {
    host: std::net::IpAddr,
    port: u16,

    config_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            port: 3000,
            config_path: String::default(),
        }
    }
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            host: from_env_or("HOST", || {
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))
            })?,
            port: from_env_or("PORT", || 3000)?,
            config_path: std::env::var("CONFIG_PATH")?,
        })
    }

    pub fn build(&self) -> anyhow::Result<Server> {
        let config = crate::config::RootConfig::from_path(&self.config_path)
            .context("reading root config")?;
        Ok(Server {
            address: std::net::SocketAddr::from((self.host, self.port)),
            root_config: crate::view::RootConfig::from(config),
        })
    }
}

#[derive(Debug)]
pub struct Server {
    address: std::net::SocketAddr,
    root_config: crate::view::RootConfig,
}

impl Server {
    #[tracing::instrument(name = "server", skip_all)]
    pub async fn run(&self) -> anyhow::Result<()> {
        let storage = chezmoi_storage::client::Config::from_env();
        let storage = storage.build().await?;
        storage.upgrade().await?;

        tracing::debug!("binding socket to {}", self.address);
        let listener = TcpListener::bind(self.address).await?;
        let app = router::create()
            .layer(Extension(storage.clone()))
            .layer(Extension(Arc::new(self.root_config.clone())))
            .layer(Extension(state::StorageWriter::new(storage)))
            .layer(TraceLayer::new_for_http());
        tracing::info!("listening on {}", self.address);
        axum::serve(listener, app).await?;
        Ok(())
    }
}
