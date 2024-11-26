use std::borrow::Cow;

use another_html_builder::{Body, Buffer};
use human_number::Formatter;

fn render_empty_last_update<'v, W: std::fmt::Write>(
    buf: Buffer<W, Body<'v>>,
) -> Buffer<W, Body<'v>> {
    buf.node("div")
        .attr((
            "class",
            "card-content align-content-center text-center min-h-150px",
        ))
        .content(|buf| buf.text("No content found"))
}

fn render_last_update<'v, W: std::fmt::Write>(
    buf: Buffer<W, Body<'v>>,
    values: &LastValues,
) -> Buffer<W, Body<'v>> {
    buf.node("div")
        .attr((
            "class",
            "card-content justify-content-center min-h-150px flex-col",
        ))
        .content(|buf| {
            buf.node("div")
                .attr(("class", "m-sm"))
                .attr(("data-label", "moisture"))
                .content(|buf| buf.raw(Formatter::si().with_unit("%").format(values.moisture)))
                .node("div")
                .attr(("class", "m-sm"))
                .attr(("data-label", "temperature"))
                .content(|buf| buf.raw(Formatter::si().with_unit("°C").format(values.temperature)))
                .node("div")
                .attr(("class", "m-sm"))
                .attr(("data-label", "brightness"))
                .content(|buf| buf.raw(Formatter::si().with_unit("lx").format(values.brightness)))
                .node("div")
                .attr(("class", "m-sm"))
                .attr(("data-label", "conductivity"))
                .content(|buf| {
                    buf.raw(
                        Formatter::si()
                            .with_unit("μS/cm")
                            .format(values.conductivity),
                    )
                })
                .node("div")
                .attr(("class", "m-sm"))
                .attr(("data-label", "battery"))
                .content(|buf| buf.raw(Formatter::si().with_unit("%").format(values.battery)))
        })
}

#[derive(Debug)]
pub struct LastValues {
    pub timestamp: u64,
    pub temperature: f64,
    pub brightness: f64,
    pub moisture: f64,
    pub conductivity: f64,
    pub battery: f64,
}

#[derive(Debug)]
pub struct MifloraCard {
    address: Cow<'static, str>,
    name: Option<Cow<'static, str>>,
    last_update: Option<LastValues>,
}

impl MifloraCard {
    pub fn new<A: Into<Cow<'static, str>>, N: Into<Cow<'static, str>>>(
        address: A,
        name: Option<N>,
        last_update: Option<LastValues>,
    ) -> Self {
        Self {
            address: address.into(),
            name: name.map(|n| n.into()),
            last_update,
        }
    }
}

impl super::prelude::Component for MifloraCard {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr(("class", "card miflora shadow min-w-250px m-md"))
            .content(|buf| {
                let buf = match self.last_update {
                    Some(ref values) => render_last_update(buf, values),
                    None => render_empty_last_update(buf),
                };
                buf.node("div")
                    .attr(("class", "card-footer"))
                    .content(|buf| {
                        buf.optional(self.name.as_deref(), |buf, name| buf.text(name).text(" - "))
                            .text(&self.address)
                    })
            })
    }
}
