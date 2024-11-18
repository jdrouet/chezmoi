use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use chezmoi_database::metrics::Metric;
use tokio::sync::mpsc::Sender;

pub(crate) mod system;

#[derive(Clone, Debug)]
pub(crate) struct RunningState(Arc<AtomicBool>);

impl RunningState {
    fn new(running: bool) -> Self {
        Self(Arc::new(AtomicBool::new(running)))
    }

    fn is_running(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }

    #[allow(unused)]
    pub(crate) fn stop(&self) {
        self.0.store(false, Ordering::Relaxed);
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Context {
    state: RunningState,
    sender: Sender<Vec<Metric>>,
}

impl Context {
    pub fn new(running: bool, sender: Sender<Vec<Metric>>) -> Self {
        Self {
            state: RunningState::new(running),
            sender,
        }
    }
}
