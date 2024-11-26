use std::borrow::Cow;

use another_html_builder::{Body, Buffer};
use human_number::Formatter;

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

    temperature_fmt: human_number::Formatter<'static>,
    brightness_fmt: human_number::Formatter<'static>,
    moisture_fmt: human_number::Formatter<'static>,
    conductivity_fmt: human_number::Formatter<'static>,
    battery_fmt: human_number::Formatter<'static>,
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

            temperature_fmt: Formatter::si().with_unit("°C"),
            brightness_fmt: Formatter::si().with_unit("lx"),
            moisture_fmt: Formatter::si().with_unit("%"),
            conductivity_fmt: Formatter::si().with_unit("μS/cm"),
            battery_fmt: Formatter::si().with_unit("%"),
        }
    }

    fn render_last_update<'v, W: std::fmt::Write>(
        &self,
        buf: Buffer<W, Body<'v>>,
    ) -> Buffer<W, Body<'v>> {
        if let Some(ref values) = self.last_update {
            buf.node("div")
                .attr((
                    "class",
                    "card-content justify-content-center min-h-150px flex-col",
                ))
                .content(|buf| {
                    buf.node("div")
                        .attr(("class", "m-sm"))
                        .attr(("data-label", "moisture"))
                        .content(|buf| buf.raw(self.moisture_fmt.format(values.moisture)))
                        .node("div")
                        .attr(("class", "m-sm"))
                        .attr(("data-label", "temperature"))
                        .content(|buf| buf.raw(self.temperature_fmt.format(values.temperature)))
                        .node("div")
                        .attr(("class", "m-sm"))
                        .attr(("data-label", "brightness"))
                        .content(|buf| buf.raw(self.brightness_fmt.format(values.brightness)))
                        .node("div")
                        .attr(("class", "m-sm"))
                        .attr(("data-label", "conductivity"))
                        .content(|buf| buf.raw(self.conductivity_fmt.format(values.conductivity)))
                        .node("div")
                        .attr(("class", "m-sm"))
                        .attr(("data-label", "battery"))
                        .content(|buf| buf.raw(self.battery_fmt.format(values.battery)))
                })
        } else {
            buf.node("div")
                .attr((
                    "class",
                    "card-content align-content-center text-center min-h-150px",
                ))
                .content(|buf| buf.text("No content found"))
        }
    }
}

impl super::prelude::Component for MifloraCard {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr(("class", "card miflora shadow min-w-250px m-md"))
            .content(|buf| {
                let buf = self.render_last_update(buf);
                buf.node("div")
                    .attr(("class", "card-footer"))
                    .content(|buf| {
                        buf.optional(self.name.as_deref(), |buf, name| buf.text(name).text(" - "))
                            .text(&self.address)
                    })
            })
    }
}
