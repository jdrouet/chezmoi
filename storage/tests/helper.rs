use chezmoi_entity::metric::Metric;
use chezmoi_storage::client::*;

pub async fn create_client() -> Client {
    let client = Config::default().build().await.unwrap();
    client.upgrade().await.unwrap();
    client
}

pub async fn create_metrics(client: &Client, values: impl Iterator<Item = &Metric>) -> u64 {
    chezmoi_storage::metric::create(client.as_ref(), values)
        .await
        .unwrap()
}
