use another_html_builder::{Body, Buffer};
use human_number::ScaledValue;

use crate::component::icon::{Icon, IconKind};
use crate::component::prelude::Component;
use crate::helper::fmt;

fn render_row<'a, W: std::fmt::Write>(
    buf: Buffer<W, Body<'a>>,
    icon: IconKind,
    name: &str,
    value: Option<(u64, ScaledValue<'a>)>,
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
            let buf = buf.optional(value, |buf, (_ts, value)| {
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
    pub temperature: Option<TimedValue>,
    pub brightness: Option<TimedValue>,
    pub moisture: Option<TimedValue>,
    pub conductivity: Option<TimedValue>,
    pub battery: Option<TimedValue>,
}

impl LastValues {
    fn is_empty(&self) -> bool {
        self.temperature.is_none()
            && self.brightness.is_none()
            && self.moisture.is_none()
            && self.conductivity.is_none()
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
                            .as_ref()
                            .map(|item| (item.timestamp, fmt::TEMPERATURE.format(item.value))),
                    );
                    let buf = render_row(
                        buf,
                        IconKind::Water,
                        "moisture",
                        self.values
                            .moisture
                            .as_ref()
                            .map(|item| (item.timestamp, fmt::PERCENTAGE.format(item.value))),
                    );
                    let buf = render_row(
                        buf,
                        IconKind::Sun,
                        "brightness",
                        self.values
                            .brightness
                            .as_ref()
                            .map(|item| (item.timestamp, fmt::BRIGHTNESS.format(item.value))),
                    );
                    let buf = render_row(
                        buf,
                        IconKind::Dashboard,
                        "conductivity",
                        self.values
                            .conductivity
                            .as_ref()
                            .map(|item| (item.timestamp, fmt::CONDUCTIVITY.format(item.value))),
                    );
                    let buf = render_row(
                        buf,
                        IconKind::Battery,
                        "battery",
                        self.values
                            .battery
                            .as_ref()
                            .map(|item| (item.timestamp, fmt::PERCENTAGE.format(item.value))),
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