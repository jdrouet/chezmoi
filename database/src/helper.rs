use std::time::SystemTime;

#[inline]
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("current time before unix epoch")
        .as_secs()
}
