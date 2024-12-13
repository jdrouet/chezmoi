use std::sync::Arc;

use chezmoi_entity::metric::Metric;
use chezmoi_storage::client::Client;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

#[derive(Clone, Debug)]
pub struct StorageWriter {
    #[allow(unused)]
    pub handler: Arc<JoinHandle<()>>,
    pub sender: tokio::sync::mpsc::Sender<Vec<Metric>>,
}

impl StorageWriter {
    pub fn new(client: Client) -> Self {
        let (sender, receiver) = mpsc::channel(20);
        Self {
            handler: Arc::new(tokio::task::spawn(async move {
                chezmoi_storage::writer::Writer::new(client)
                    .run(receiver)
                    .await
            })),
            sender,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StorageReader {
    client: Client,
}

impl AsRef<Client> for StorageReader {
    fn as_ref(&self) -> &Client {
        &self.client
    }
}

impl StorageReader {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn ping(&self) -> Result<(), chezmoi_storage::sqlx::Error> {
        self.client.ping().await
    }
}
