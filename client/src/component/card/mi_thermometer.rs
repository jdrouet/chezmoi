use another_html_builder::{Body, Buffer};
use human_number::ScaledValue;

use crate::component::icon::{Icon, IconKind};
use crate::component::prelude::Component;
use crate::helper::fmt;

fn render_row<'a, W: std::fmt::Write>(
    buf: Buffer<W, Body<'a>>,
    icon: IconKind,
    name: &str,
    value: Option<ScaledValue<'a>>,
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
            let buf = buf.optional(value, |buf, value| {
                buf.node("label").content(|buf| buf.raw(value))
            });
            buf
        })
}

#[derive(Clone, Copy, Debug)]
pub struct TimedValue {
    pub timestamp: u64,
    pub value: f64,
}

impl From<(u64, f64)> for TimedValue {
    fn from((timestamp, value): (u64, f64)) -> Self {
        Self { timestamp, value }
    }
}

#[derive(Debug)]
pub struct LastValues {
    pub timestamp: Option<u64>,
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
    pub battery: Option<f64>,
}

impl LastValues {
    fn is_empty(&self) -> bool {
        self.timestamp.is_none()
            && self.temperature.is_none()
            && self.humidity.is_none()
            && self.battery.is_none()
    }
}

#[derive(Debug)]
pub struct Card<'a> {
    address: &'a str,
    name: Option<&'a str>,
    values: LastValues,
}

impl<'a> Card<'a> {
    pub fn new(address: &'a str, name: Option<&'a str>, values: LastValues) -> Self {
        Self {
            address,
            name,
            values,
        }
    }

    fn render_last_update<'v, W: std::fmt::Write>(
        &self,
        buf: Buffer<W, Body<'v>>,
    ) -> Buffer<W, Body<'v>> {
        if self.values.is_empty() {
            buf.node("div")
                .attr((
                    "class",
                    "card-content align-content-center text-center min-h-150px",
                ))
                .content(|buf| buf.text("No content found"))
        } else {
            buf.node("div")
                .attr(("class", "card-content min-h-150px flex-col py-md"))
                .content(|buf| {
                    let buf = render_row(
                        buf,
                        IconKind::TemperatureHot,
                        "temperature",
                        self.values
                            .temperature
                            .map(|item| fmt::TEMPERATURE.format(item)),
                    );
                    let buf = render_row(
                        buf,
                        IconKind::Water,
                        "moisture",
                        self.values
                            .humidity
                            .map(|item| fmt::PERCENTAGE.format(item)),
                    );
                    let buf = render_row(
                        buf,
                        IconKind::Battery,
                        "battery",
                        self.values.battery.map(|item| fmt::PERCENTAGE.format(item)),
                    );
                    buf
                })
        }
    }
}

impl<'a> Component for Card<'a> {
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
