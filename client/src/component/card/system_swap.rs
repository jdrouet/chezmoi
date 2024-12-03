use another_html_builder::{Body, Buffer};

#[derive(Debug)]
pub struct Card(super::binary_usage::Card<'static>);

impl Card {
    pub fn new(total: Option<f64>, used: Option<f64>) -> Self {
        Self(super::binary_usage::Card::new("Swap", total, used))
    }
}

impl crate::component::prelude::Component for Card {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        self.0.render(buf)
    }
}
