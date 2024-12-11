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

#[derive(Debug)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> From<T> for OneOrMany<T> {
    fn from(value: T) -> Self {
        Self::One(value)
    }
}

impl<T> From<Vec<T>> for OneOrMany<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Many(value)
    }
}

impl<T> OneOrMany<T> {
    pub fn len(&self) -> usize {
        match self {
            Self::One(_) => 1,
            Self::Many(inner) => inner.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::One(_) => false,
            Self::Many(inner) => inner.is_empty(),
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        match self {
            Self::One(item) => vec![item],
            Self::Many(inner) => inner,
        }
    }
}
