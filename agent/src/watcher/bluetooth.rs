use std::collections::HashSet;

use anyhow::Context;
use bluer::{Adapter, AdapterEvent, Address, DeviceEvent, DeviceProperty, DiscoveryFilter};
use futures::stream::SelectAll;
use futures::Stream;
use futures::{pin_mut, StreamExt};
use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub enum WatcherEvent {
    DeviceAdded(Address),
    DeviceRemoved(Address),
    DeviceChanged(Address, DeviceProperty),
}

#[derive(Debug)]
pub(crate) struct Watcher {
    adapter: Adapter,
    follow_changes: HashSet<Address>,
}

impl Watcher {
    pub fn new(adapter: Adapter, follow_changes: HashSet<Address>) -> Self {
        Self {
            adapter,
            follow_changes,
        }
    }

    async fn listen(
        &self,
        ctx: &crate::sensor::Context,
        device_events: impl Stream<Item = AdapterEvent>,
        sender: &broadcast::Sender<WatcherEvent>,
    ) -> anyhow::Result<()> {
        pin_mut!(device_events);

        let mut all_change_events = SelectAll::new();

        while ctx.state.is_running() {
            tokio::select! {
                Some(device_event) = device_events.next() => {
                    match device_event {
                        AdapterEvent::DeviceAdded(addr) => {
                            if let Err(err) = sender.send(WatcherEvent::DeviceAdded(addr)) {
                                tracing::error!(message = "unable to forward added device", address = %addr, error = %err);
                            }
                            if self.follow_changes.contains(&addr) {
                                if let Ok(device) = self.adapter.device(addr) {
                                    let change_events = device.events().await?.map(move |evt| (addr, evt));
                                    all_change_events.push(change_events);
                                }
                            }
                        }
                        AdapterEvent::DeviceRemoved(addr) => {
                            if let Err(err) = sender.send(WatcherEvent::DeviceRemoved(addr)) {
                                tracing::error!(message = "unable to forward removed device", address = %addr, error = %err);
                            }
                        }
                        _ => (),
                    }
                }
                Some((addr, DeviceEvent::PropertyChanged(property))) = all_change_events.next() => {
                    if let Err(err) = sender.send(WatcherEvent::DeviceChanged(addr, property)) {
                        tracing::error!(message = "unable to forward changed device", address = %addr, error = %err);
                    }
                }
                else => break
            }
        }

        Ok(())
    }

    pub async fn execute(
        &self,
        ctx: &crate::sensor::Context,
        sender: &broadcast::Sender<WatcherEvent>,
    ) -> anyhow::Result<()> {
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
            Ok(events) => self.listen(ctx, events, sender).await,
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
                self.listen(ctx, events, sender).await
            }
            other => other.map(|_| ()).context("discovering device events"),
        }
    }

    #[tracing::instrument(name = "bluetooth_watcher", skip_all, fields(adapter = %self.adapter.name()))]
    pub async fn run(
        self,
        ctx: crate::sensor::Context,
        sender: broadcast::Sender<WatcherEvent>,
    ) -> anyhow::Result<()> {
        while ctx.state.is_running() {
            if let Err(err) = self.execute(&ctx, &sender).await {
                tracing::error!(message = "something went wrong", error = %err, cause = ?err.source());
            }
        }
        Ok(())
    }
}
