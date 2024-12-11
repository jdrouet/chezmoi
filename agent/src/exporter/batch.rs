use std::time::Duration;

use chezmoi_entity::metric::Metric;
use chezmoi_entity::OneOrMany;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
enum FlushOrigin {
    Timer,
    Capacity,
}

#[derive(Debug)]
pub struct BatchExporter<H> {
    interval: Duration,
    capacity: usize,
    handler: H,
}

impl<H: super::prelude::Handler> BatchExporter<H> {
    pub fn new(handler: H) -> Self {
        Self {
            interval: Duration::new(30, 0),
            capacity: 50,
            handler,
        }
    }

    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    pub fn set_interval(&mut self, interval: Duration) -> &mut Self {
        self.interval = interval;
        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn set_capacity(&mut self, capacity: usize) -> &mut Self {
        self.capacity = capacity;
        self
    }
}

impl<H: super::prelude::Handler> BatchExporter<H> {
    #[tracing::instrument(name = "flush", skip_all, fields(count = values.len(), reason = ?reason))]
    async fn flush(&mut self, reason: FlushOrigin, values: Vec<Metric>) {
        self.handler.handle(OneOrMany::Many(values)).await;
    }
}

impl<H: super::prelude::Handler + Send> super::prelude::Exporter for BatchExporter<H> {
    #[tracing::instrument(name = "batch", skip_all)]
    async fn run(mut self, mut receiver: Receiver<OneOrMany<Metric>>) {
        let mut flush_ticker = tokio::time::interval(self.interval);
        let mut buffer: Vec<Metric> = Vec::with_capacity(self.capacity);
        while !receiver.is_closed() {
            tokio::select! {
                _ = flush_ticker.tick() => {
                    if !buffer.is_empty() {
                        let mut new_buffer = Vec::with_capacity(self.capacity);
                        std::mem::swap(&mut buffer, &mut new_buffer);
                        self.flush(FlushOrigin::Timer, new_buffer).await;
                    }
                },
                Some(next) = receiver.recv() => {
                    match next {
                        OneOrMany::One(value) => buffer.push(value),
                        OneOrMany::Many(values) => buffer.extend(values.into_iter()),
                    };
                    if buffer.len() >= self.capacity {
                        let mut new_buffer = Vec::with_capacity(self.capacity);
                        std::mem::swap(&mut buffer, &mut new_buffer);
                        self.flush(FlushOrigin::Capacity, new_buffer).await;
                        flush_ticker.reset();
                    }
                }
                else => break,
            }
        }
    }
}
