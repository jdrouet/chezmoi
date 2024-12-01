use another_html_builder::{Body, Buffer};

use crate::helper::fmt;

#[derive(Debug)]
pub struct Card {
    title: &'static str,
    total: Option<f64>,
    used: Option<f64>,
}

impl Card {
    pub fn new(title: &'static str, total: Option<f64>, used: Option<f64>) -> Self {
        Self { title, total, used }
    }
}

impl crate::component::prelude::Component for Card {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr(("class", "card shadow x-sm y-sm m-md flex-col"))
            .content(|buf| {
                buf.node("div")
                    .attr((
                        "class",
                        "card-content flex-1 text-center align-content-center py-md",
                    ))
                    .content(|buf| {
                        buf.node("p")
                            .attr(("class", "text-xl"))
                            .content(|buf| match (self.used, self.total) {
                                (Some(used), Some(total)) => {
                                    let percent = used * 100.0 / total;
                                    buf.raw(fmt::PERCENTAGE.format(percent))
                                }
                                _ => buf.text("-"),
                            })
                            .node("p")
                            .content(|buf| {
                                let buf = match self.used {
                                    Some(value) => buf.raw(fmt::BYTES.format(value)),
                                    None => buf.text("-"),
                                };
                                let buf = buf.text(" / ");
                                let buf = match self.total {
                                    Some(value) => buf.raw(fmt::BYTES.format(value)),
                                    None => buf.text("-"),
                                };
                                buf
                            })
                    })
                    .node("div")
                    .attr(("class", "card-footer"))
                    .content(|buf| buf.text(self.title))
            })
    }
}
