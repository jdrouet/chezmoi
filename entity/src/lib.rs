use std::borrow::Cow;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod metric;

pub type CowStr<'a> = Cow<'a, str>;

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
