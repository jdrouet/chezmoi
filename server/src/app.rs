use tower_http::trace::TraceLayer;

pub(crate) struct Config {
    host: std::net::IpAddr,
    port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            host: crate::helper::parse_env_or(
                "HOST",
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            )?,
            port: crate::helper::parse_env_or("PORT", 3000)?,
        })
    }

    pub async fn build(self) -> anyhow::Result<Application> {
        Ok(Application {
            socket_address: std::net::SocketAddr::from((self.host, self.port)),
        })
    }
}

pub(crate) struct Application {
    socket_address: std::net::SocketAddr,
}

impl Application {
    fn router(&self) -> axum::Router {
        crate::router::create().layer(TraceLayer::new_for_http())
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::debug!("binding socket to {}", self.socket_address);
        let listener = tokio::net::TcpListener::bind(self.socket_address).await?;
        tracing::info!("listening on {}", self.socket_address);
        axum::serve(listener, self.router()).await?;
        Ok(())
    }
}
