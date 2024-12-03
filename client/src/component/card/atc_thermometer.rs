use another_html_builder::{Body, Buffer};
use human_number::ScaledValue;

use crate::component::helper::format_datetime;
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
            buf.node("label").content(|buf| match value {
                Some(value) => buf.raw(value),
                None => buf.text("-"),
            })
        })
}

fn render_date_row<'a, W: std::fmt::Write>(
    buf: Buffer<W, Body<'a>>,
    icon: IconKind,
    name: &str,
    value: Option<u64>,
) -> Buffer<W, Body<'a>> {
    buf.node("div")
        .attr(("class", "flex-row mx-md my-sm"))
        .attr(("data-label", name))
        .content(|buf| {
            let buf = Icon::new(icon).render(buf);

            buf.node("label")
                .attr(("class", "flex-1 mx-sm"))
                .content(|buf| match value.and_then(format_datetime) {
                    Some(dt) => buf.raw(dt),
                    None => buf.text("-"),
                })
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
pub struct Values {
    pub timestamp: Option<u64>,
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
    pub battery: Option<f64>,
}

#[derive(Debug)]
pub struct Card<'a> {
    address: &'a str,
    name: Option<&'a str>,
    values: Values,
}

impl<'a> Card<'a> {
    pub fn new(address: &'a str, name: Option<&'a str>, values: Values) -> Self {
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
        buf.node("div")
            .attr(("class", "card-content flex-col flex-1 py-md"))
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
                let buf = render_date_row(buf, IconKind::Time, "timestamp", self.values.timestamp);
                buf
            })
    }
}

impl<'a> Component for Card<'a> {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr(("class", "card x-sm y-sm shadow flex-col m-md"))
            .content(|buf| {
                let buf = self.render_last_update(buf);
                buf.node("div")
                    .attr(("class", "card-footer"))
                    .content(|buf| {
                        if let Some(name) = self.name {
                            buf.node("b")
                                .content(|buf| buf.text(name))
                                .text(" - ")
                                .node("i")
                                .content(|buf| buf.text(self.address))
                        } else {
                            buf.node("i").content(|buf| buf.text(self.address))
                        }
                    })
            })
    }
}
