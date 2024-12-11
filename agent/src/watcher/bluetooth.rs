use std::collections::HashSet;

use anyhow::Context;
use bluer::{Adapter, AdapterEvent, Address, DeviceEvent, DeviceProperty, DiscoveryFilter};
use futures::stream::SelectAll;
use futures::Stream;
use futures::{pin_mut, StreamExt};
use tokio::sync::broadcast;

const fn default_channel_size() -> usize {
    50
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    adapter: Option<String>,
    #[serde(default = "default_channel_size")]
    channel_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            adapter: None,
            channel_size: default_channel_size(),
        }
    }
}

impl Config {
    pub async fn build(
        &self,
        follow: impl Iterator<Item = bluer::Address>,
    ) -> anyhow::Result<(Watcher, broadcast::Receiver<WatcherEvent>)> {
        let (sender, receiver) = broadcast::channel(self.channel_size);

        let session = bluer::Session::new().await?;
        let adapter = match self.adapter.as_deref() {
            Some(name) => session.adapter(name)?,
            None => session.default_adapter().await?,
        };

        Ok((
            Watcher {
                adapter,
                follow: HashSet::from_iter(follow),
                sender,
            },
            receiver,
        ))
    }
}

#[derive(Debug)]
pub struct Watcher {
    adapter: Adapter,
    follow: HashSet<Address>,
    sender: broadcast::Sender<WatcherEvent>,
}

impl Watcher {
    async fn listen(&self, device_events: impl Stream<Item = AdapterEvent>) -> anyhow::Result<()> {
        pin_mut!(device_events);

        let mut all_change_events = SelectAll::new();

        loop {
            tokio::select! {
                Some(device_event) = device_events.next() => {
                    match device_event {
                        AdapterEvent::DeviceAdded(addr) => {
                            if let Err(err) = self.sender.send(WatcherEvent::DeviceAdded(addr)) {
                                tracing::error!(message = "unable to forward added device", address = %addr, error = %err);
                            }
                            if self.follow.contains(&addr) {
                                if let Ok(device) = self.adapter.device(addr) {
                                    let change_events = device.events().await?.map(move |evt| (addr, evt));
                                    all_change_events.push(change_events);
                                }
                            }
                        }
                        AdapterEvent::DeviceRemoved(addr) => {
                            if let Err(err) = self.sender.send(WatcherEvent::DeviceRemoved(addr)) {
                                tracing::error!(message = "unable to forward removed device", address = %addr, error = %err);
                            }
                        }
                        _ => (),
                    }
                }
                Some((addr, DeviceEvent::PropertyChanged(property))) = all_change_events.next() => {
                    if let Err(err) = self.sender.send(WatcherEvent::DeviceChanged(addr, property)) {
                        tracing::error!(message = "unable to forward changed device", address = %addr, error = %err);
                    }
                }
                else => break
            }
        }

        Ok(())
    }

    async fn execute(&self) -> anyhow::Result<()> {
        if !self.adapter.is_powered().await? {
            self.adapter
                .set_powered(true)
                .await
                .context("powering bluetooth adapter")?;
        }

        self.adapter
            .set_discovery_filter(DiscoveryFilter::default())
            .await
            .context("setting discovery filter")?;

        match self.adapter.discover_devices().await {
            Ok(events) => self.listen(events).await,
            Err(bluer::Error {
                kind: bluer::ErrorKind::InProgress,
                message: _,
            }) => {
                tracing::debug!("discovery already in progress, listening to existing");
                let events = self
                    .adapter
                    .events()
                    .await
                    .context("listening device events")?;
                self.listen(events).await
            }
            other => other.map(|_| ()).context("discovering device events"),
        }
    }
}

impl crate::prelude::Worker for Watcher {
    #[tracing::instrument(name = "bluetooth_watcher", skip_all, fields(adapter = %self.adapter.name()))]
    async fn run(self) -> anyhow::Result<()> {
        self.execute().await
    }
}

#[derive(Clone, Debug)]
pub enum WatcherEvent {
    DeviceAdded(Address),
    DeviceRemoved(Address),
    DeviceChanged(Address, DeviceProperty),
}
