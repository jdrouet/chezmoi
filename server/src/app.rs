use std::sync::Arc;

use axum::Extension;
use tower_http::trace::TraceLayer;

use crate::service::dashboard::Dashboard;

fn default_host() -> std::net::IpAddr {
    std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))
}

fn default_port() -> u16 {
    3000
}

fn default_assets_path() -> String {
    String::from("./assets")
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Config {
    #[serde(default = "default_host")]
    host: std::net::IpAddr,
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "default_assets_path")]
    assets_path: String,
    #[serde(default)]
    dashboard: Dashboard,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            assets_path: default_assets_path(),
            dashboard: Default::default(),
        }
    }
}

impl Config {
    pub async fn build(self) -> anyhow::Result<Application> {
        Ok(Application {
            assets_path: self.assets_path,
            dashboard: Arc::new(self.dashboard),
            socket_address: std::net::SocketAddr::from((self.host, self.port)),
        })
    }
}

pub(crate) struct Application {
    assets_path: String,
    dashboard: Arc<Dashboard>,
    socket_address: std::net::SocketAddr,
}

impl Application {
    fn router(&self, database: chezmoi_database::Client) -> axum::Router {
        crate::router::create(&self.assets_path)
            .layer(Extension(database))
            .layer(Extension(self.dashboard.clone()))
            .layer(TraceLayer::new_for_http())
    }

    pub async fn run(self, database: chezmoi_database::Client) -> anyhow::Result<()> {
        tracing::debug!("binding socket to {}", self.socket_address);
        let listener = tokio::net::TcpListener::bind(self.socket_address).await?;
        tracing::info!("listening on {}", self.socket_address);
        axum::serve(listener, self.router(database)).await?;
        Ok(())
    }
}
