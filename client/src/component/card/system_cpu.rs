use another_html_builder::{Body, Buffer};

use crate::helper::fmt;

#[derive(Debug)]
pub struct Card {
    usage: Option<f64>,
}

impl Card {
    pub fn new(usage: Option<f64>) -> Self {
        Self { usage }
    }
}

impl crate::component::prelude::Component for Card {
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
                        buf.node("p")
                            .attr(("class", "text-xl"))
                            .content(|buf| match self.usage {
                                Some(value) => buf.raw(fmt::PERCENTAGE.format(value)),
                                None => buf.text(" - "),
                            })
                    })
                    .node("div")
                    .attr(("class", "card-footer"))
                    .content(|buf| buf.text("CPU"))
            })
    }
}
