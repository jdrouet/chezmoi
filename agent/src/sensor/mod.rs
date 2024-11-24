use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use chezmoi_database::metrics::entity::Metric;
use tokio::sync::mpsc::Sender;

#[cfg(feature = "bluetooth")]
pub(crate) mod bluetooth;
pub(crate) mod system;

#[derive(Clone, Debug)]
pub(crate) struct Hostname(Option<Arc<String>>);

impl Default for Hostname {
    fn default() -> Self {
        Self(sysinfo::System::host_name().map(Arc::new))
    }
}

impl Hostname {
    pub fn inner(&self) -> Option<Arc<String>> {
        self.0.clone()
    }
}

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
