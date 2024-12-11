pub trait Worker: Send + Sized + Sync {
    fn run(self) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
