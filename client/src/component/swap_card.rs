use another_html_builder::{Body, Buffer};

use super::binary_usage_card::BinaryUsageCard;

#[derive(Debug)]
pub struct SwapCard(BinaryUsageCard);

impl SwapCard {
    pub fn new(total: f64, used: f64) -> Self {
        Self(BinaryUsageCard::new("Swap", total, used))
    }
}

impl super::prelude::Component for SwapCard {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        self.0.render(buf)
    }
}
