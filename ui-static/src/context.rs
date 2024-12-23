use crate::helper::attributes::Concat;

pub struct Context {
    pub base_url: &'static str,
}

impl Default for Context {
    fn default() -> Self {
        Self { base_url: "" }
    }
}

impl Context {
    pub const fn new(base_url: &'static str) -> Self {
        Self { base_url }
    }

    pub const fn relative() -> Self {
        Self::new("")
    }

    pub const fn absolute() -> Self {
        Self::new("/")
    }

    pub fn url<'a>(&self, rest: &'a str) -> Concat<&'static str, &'a str> {
        Concat(self.base_url, rest)
    }
}
