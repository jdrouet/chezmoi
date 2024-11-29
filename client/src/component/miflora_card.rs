use std::borrow::Cow;

use another_html_builder::{Body, Buffer};
use human_number::ScaledValue;

use super::prelude::Component;
use crate::component::icon::{Icon, IconKind};
use crate::helper::fmt;

fn render_row<'a, W: std::fmt::Write>(
    buf: Buffer<W, Body<'a>>,
    icon: IconKind,
    name: &str,
    value: ScaledValue<'a>,
) -> Buffer<W, Body<'a>> {
    buf.node("div")
        .attr(("class", "flex-row mx-md my-sm"))
        .attr(("data-label", name))
        .content(|buf| {
            let buf = Icon::new(icon).render(buf);
            let buf = buf
                .node("label")
                .attr(("class", "flex-1 mx-sm"))
                .content(|buf| buf.text(name));
            let buf = buf.node("label").content(|buf| buf.raw(value));
            buf
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

    fn render_last_update<'v, W: std::fmt::Write>(
        &self,
        buf: Buffer<W, Body<'v>>,
    ) -> Buffer<W, Body<'v>> {
        if let Some(ref values) = self.last_update {
            buf.node("div")
                .attr(("class", "card-content min-h-150px flex-col py-md"))
                .content(|buf| {
                    let buf = render_row(
                        buf,
                        IconKind::Water,
                        "moisture",
                        fmt::PERCENTAGE.format(values.moisture),
                    );
                    let buf = render_row(
                        buf,
                        IconKind::TemperatureHot,
                        "temperature",
                        fmt::TEMPERATURE.format(values.temperature),
                    );
                    let buf = render_row(
                        buf,
                        IconKind::Sun,
                        "brightness",
                        fmt::BRIGHTNESS.format(values.brightness),
                    );
                    let buf = render_row(
                        buf,
                        IconKind::Dashboard,
                        "conductivity",
                        fmt::CONDUCTIVITY.format(values.conductivity),
                    );
                    let buf = render_row(
                        buf,
                        IconKind::Battery,
                        "battery",
                        fmt::PERCENTAGE.format(values.battery),
                    );
                    buf
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
                        if let Some(ref name) = self.name {
                            buf.node("b")
                                .content(|buf| buf.text(name))
                                .text(" - ")
                                .node("i")
                                .content(|buf| buf.text(&self.address))
                        } else {
                            buf.node("i").content(|buf| buf.text(&self.address))
                        }
                    })
            })
    }
}
