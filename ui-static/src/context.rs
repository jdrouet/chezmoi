use crate::helper::attributes::Concat;

pub struct Context {
    pub base_url: &'static str,
    pub timerange: (u64, u64),
}

impl Default for Context {
    fn default() -> Self {
        Self {
            base_url: "",
            timerange: (0, 60 * 60 * 24),
        }
    }
}

impl Context {
    pub const fn new(base_url: &'static str, timerange: (u64, u64)) -> Self {
        Self {
            base_url,
            timerange,
        }
    }

    pub const fn relative(timerange: (u64, u64)) -> Self {
        Self::new("", timerange)
    }

    pub const fn absolute(timerange: (u64, u64)) -> Self {
        Self::new("/", timerange)
    }

    pub fn url<'a>(&self, rest: &'a str) -> Concat<&'static str, &'a str> {
        Concat(self.base_url, rest)
    }
}
