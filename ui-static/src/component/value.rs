#[derive(Clone, Debug)]
pub struct TimedValue {
    pub timestamp: u64,
    pub value: f64,
}

impl TimedValue {
    pub fn new(timestamp: u64, value: f64) -> Self {
        Self { timestamp, value }
    }
}
