use axum::Extension;
use tokio::net::TcpListener;

mod router;
mod state;

#[derive(Debug)]
pub struct Config {
    host: std::net::IpAddr,
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            port: 3000,
        }
    }
}

impl Config {
    pub fn build(&self) -> Server {
        Server {
            address: std::net::SocketAddr::from((self.host, self.port)),
        }
    }
}

#[derive(Debug)]
pub struct Server {
    address: std::net::SocketAddr,
}

impl Server {
    #[tracing::instrument(name = "server", skip_all)]
    pub async fn run(&self) -> anyhow::Result<()> {
        let storage = chezmoi_storage::client::Config::default();
        let storage = storage.build().await?;

        tracing::debug!("binding socket to {}", self.address);
        let listener = TcpListener::bind(self.address).await?;
        let app = router::create().layer(Extension(state::StorageWriter::new(storage)));
        tracing::info!("listening on {}", self.address);
        axum::serve(listener, app).await?;
        Ok(())
    }
}
