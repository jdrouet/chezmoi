use another_html_builder::{Body, Buffer};

use crate::helper::fmt;

#[derive(Debug)]
pub struct BinaryUsageCard {
    title: &'static str,
    total: f64,
    used: f64,
}

impl BinaryUsageCard {
    pub fn new(title: &'static str, total: f64, used: f64) -> Self {
        Self { title, total, used }
    }
}

impl super::prelude::Component for BinaryUsageCard {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr((
                "class",
                "card memory-usage shadow min-w-250px h-150px m-md flex-col",
            ))
            .content(|buf| {
                buf.node("div")
                    .attr((
                        "class",
                        "card-content flex-1 text-center align-content-center",
                    ))
                    .content(|buf| {
                        let percent = self.used * 100.0 / self.total;
                        buf.node("p")
                            .attr(("class", "text-xl"))
                            .content(|buf| buf.raw(fmt::PERCENTAGE.format(percent)))
                            .node("p")
                            .content(|buf| {
                                buf.raw(fmt::BYTES.format(self.used))
                                    .text(" / ")
                                    .raw(fmt::BYTES.format(self.total))
                            })
                    })
                    .node("div")
                    .attr(("class", "card-footer"))
                    .content(|buf| buf.text(self.title))
            })
    }
}
