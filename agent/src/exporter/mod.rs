use std::time::Duration;

use chezmoi_entity::metric::Metric;
use tokio::sync::mpsc;

use crate::collector::prelude::OneOrMany;

pub mod prelude;
pub mod trace;

#[derive(Debug)]
enum FlushOrigin {
    Timer,
    Capacity,
}

#[derive(Debug)]
pub struct Exporter<T> {
    flush_interval: Duration,
    flush_capacity: usize,
    target: T,
}

impl<T: prelude::Target> Exporter<T> {
    pub fn new(target: T) -> Self {
        Self {
            flush_interval: Duration::new(30, 0),
            flush_capacity: 50,
            target,
        }
    }

    pub fn with_flush_interval(mut self, flush_interval: Duration) -> Self {
        self.flush_interval = flush_interval;
        self
    }

    pub fn set_flush_interval(&mut self, flush_interval: Duration) -> &mut Self {
        self.flush_interval = flush_interval;
        self
    }

    pub fn with_flush_capacity(mut self, flush_capacity: usize) -> Self {
        self.flush_capacity = flush_capacity;
        self
    }

    pub fn set_flush_capacity(&mut self, flush_capacity: usize) -> &mut Self {
        self.flush_capacity = flush_capacity;
        self
    }
}

impl<T: prelude::Target> Exporter<T> {
    #[tracing::instrument(name = "flush", skip_all, fields(count = values.len(), reason = ?reason))]
    async fn flush(&self, reason: FlushOrigin, values: Vec<Metric>) {
        if let Err(err) = self.target.flush(values).await {
            tracing::warn!(message = "unable to forward metrics", error = %err);
        }
    }

    #[tracing::instrument(name = "collector", skip_all)]
    pub async fn run(&self, mut receiver: mpsc::Receiver<OneOrMany<Metric>>) {
        let mut flush_ticker = tokio::time::interval(self.flush_interval);
        let mut buffer: Vec<Metric> = Vec::with_capacity(self.flush_capacity);
        while !receiver.is_closed() {
            tokio::select! {
                _ = flush_ticker.tick() => {
                    if !buffer.is_empty() {
                        let mut new_buffer = Vec::with_capacity(self.flush_capacity);
                        std::mem::swap(&mut buffer, &mut new_buffer);
                        self.flush(FlushOrigin::Timer, new_buffer).await;
                    }
                },
                Some(next) = receiver.recv() => {
                    match next {
                        OneOrMany::One(value) => buffer.push(value),
                        OneOrMany::Many(values) => buffer.extend(values.into_iter()),
                    };
                    if buffer.len() >= self.flush_capacity {
                        let mut new_buffer = Vec::with_capacity(self.flush_capacity);
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
