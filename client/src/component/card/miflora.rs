use another_html_builder::{AttributeValue, Body, Buffer};
use human_number::ScaledValue;

use crate::component::helper::format_datetime;
use crate::component::icon::{Icon, IconKind};
use crate::component::prelude::Component;
use crate::helper::fmt;

fn empty<'a, W: std::fmt::Write>(buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>> {
    buf
}

fn render_state_icon<'a, W: std::fmt::Write>(
    buf: Buffer<W, Body<'a>>,
    state: ValueState,
) -> Buffer<W, Body<'a>> {
    match state {
        ValueState::Normal => buf,
        ValueState::Low { min: _ } => buf
            .node("i")
            .attr(("class", "ri-arrow-up-s-line"))
            .content(empty),
        ValueState::High { max: _ } => buf
            .node("i")
            .attr(("class", "ri-arrow-down-s-line"))
            .content(empty),
    }
}

fn render_row<'a, W: std::fmt::Write>(
    buf: Buffer<W, Body<'a>>,
    icon: IconKind,
    name: &str,
    value: Option<(u64, ScaledValue<'a>, ValueState)>,
) -> Buffer<W, Body<'a>> {
    let classname = match value {
        Some((_, _, ValueState::High { .. })) | Some((_, _, ValueState::Low { .. })) => {
            "flex-row mx-md my-sm text-error"
        }
        _ => "flex-row mx-md my-sm",
    };
    buf.node("div")
        .attr(("class", classname))
        .attr(("data-label", name))
        .content(|buf| {
            let buf = Icon::new(icon).render(buf);
            let buf = buf
                .node("label")
                .attr(("class", "flex-1 mx-sm"))
                .content(|buf| buf.text(name));

            match value {
                Some((_ts, value, state)) => {
                    let buf = render_state_icon(buf, state);
                    buf.node("label").content(|buf| buf.raw(value))
                }
                None => buf.node("label").content(|buf| buf.text("-")),
            }
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
            let buf = buf
                .node("label")
                .attr(("class", "flex-1 mx-sm"))
                .content(|buf| buf.text(name));

            buf.node("label")
                .content(|buf| match value.and_then(format_datetime) {
                    Some(dt) => buf.raw(dt),
                    None => buf.text("-"),
                })
        })
}

#[derive(Clone, Copy, Debug)]
pub enum ValueState {
    Low { min: f64 },
    Normal,
    High { max: f64 },
}

#[derive(Clone, Copy, Debug)]
pub struct TimedValue {
    pub timestamp: u64,
    pub value: f64,
    pub state: ValueState,
}

impl From<(u64, f64, ValueState)> for TimedValue {
    fn from((timestamp, value, state): (u64, f64, ValueState)) -> Self {
        Self {
            timestamp,
            value,
            state,
        }
    }
}

#[derive(Debug)]
pub struct Values {
    pub temperature: Option<TimedValue>,
    pub brightness: Option<TimedValue>,
    pub moisture: Option<TimedValue>,
    pub conductivity: Option<TimedValue>,
    pub battery: Option<TimedValue>,
}

impl Values {
    pub fn last_timestamp(&self) -> Option<u64> {
        self.temperature
            .map(|v| v.timestamp)
            .into_iter()
            .chain(self.brightness.map(|v| v.timestamp).into_iter())
            .chain(self.moisture.map(|v| v.timestamp).into_iter())
            .chain(self.conductivity.map(|v| v.timestamp).into_iter())
            .chain(self.battery.map(|v| v.timestamp).into_iter())
            .max()
    }
}

struct CardImageStyle<'a>(&'a str);

impl AttributeValue for CardImageStyle<'_> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "background-image: url({})", self.0)
    }
}

#[derive(Debug)]
pub struct Card<'a> {
    address: &'a str,
    name: Option<&'a str>,
    image: Option<&'a str>,
    values: Values,
}

impl<'a> Card<'a> {
    pub fn new(
        address: &'a str,
        name: Option<&'a str>,
        image: Option<&'a str>,
        values: Values,
    ) -> Self {
        Self {
            address,
            name,
            image,
            values,
        }
    }

    fn render_picture<'v, W: std::fmt::Write>(
        &self,
        buf: Buffer<W, Body<'v>>,
    ) -> Buffer<W, Body<'v>> {
        match self.image {
            Some(url) => buf
                .node("div")
                .attr(("class", "card-image flex-1"))
                .attr(("style", CardImageStyle(url)))
                .content(empty),
            None => buf
                .node("div")
                .attr((
                    "class",
                    "card-image flex-1 text-xxl text-center align-content-center",
                ))
                .content(|buf| buf.text("🪴")),
        }
    }

    fn render_values<'v, W: std::fmt::Write>(
        &self,
        buf: Buffer<W, Body<'v>>,
    ) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr(("class", "card-content flex-col py-md"))
            .content(|buf| {
                let buf = render_row(
                    buf,
                    IconKind::TemperatureHot,
                    "temperature",
                    self.values.temperature.as_ref().map(|item| {
                        (
                            item.timestamp,
                            fmt::TEMPERATURE.format(item.value),
                            item.state,
                        )
                    }),
                );
                let buf = render_row(
                    buf,
                    IconKind::Water,
                    "moisture",
                    self.values.moisture.as_ref().map(|item| {
                        (
                            item.timestamp,
                            fmt::PERCENTAGE.format(item.value),
                            item.state,
                        )
                    }),
                );
                let buf = render_row(
                    buf,
                    IconKind::Sun,
                    "brightness",
                    self.values.brightness.as_ref().map(|item| {
                        (
                            item.timestamp,
                            fmt::BRIGHTNESS.format(item.value),
                            item.state,
                        )
                    }),
                );
                let buf = render_row(
                    buf,
                    IconKind::Dashboard,
                    "conductivity",
                    self.values.conductivity.as_ref().map(|item| {
                        (
                            item.timestamp,
                            fmt::CONDUCTIVITY.format(item.value),
                            item.state,
                        )
                    }),
                );
                let buf = render_row(
                    buf,
                    IconKind::Battery,
                    "battery",
                    self.values.battery.as_ref().map(|item| {
                        (
                            item.timestamp,
                            fmt::PERCENTAGE.format(item.value),
                            item.state,
                        )
                    }),
                );
                let buf = render_date_row(
                    buf,
                    IconKind::Time,
                    "timestamp",
                    self.values.last_timestamp(),
                );
                buf
            })
    }

    fn render_footer<'v, W: std::fmt::Write>(
        &self,
        buf: Buffer<W, Body<'v>>,
    ) -> Buffer<W, Body<'v>> {
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
    }
}

impl<'a> Component for Card<'a> {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr(("class", "card x-md y-md shadow m-md flex-col"))
            .content(|buf| {
                let buf = self.render_picture(buf);
                let buf = self.render_values(buf);
                self.render_footer(buf)
            })
    }
}
