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
            .attr(("class", "card x-sm y-sm shadow m-md flex-col"))
            .content(|buf| {
                buf.node("div")
                    .attr((
                        "class",
                        "card-content flex-1 text-center align-content-center py-md",
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
