use another_html_builder::{AttributeValue, Body, Buffer};

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

fn render_state_tooltip_content<'a, W: std::fmt::Write>(
    buf: Buffer<W, Body<'a>>,
    state: ValueState,
    formatter: &human_number::Formatter<'_>,
) -> Buffer<W, Body<'a>> {
    match state {
        ValueState::Normal => buf,
        ValueState::Low { min } => buf
            .node("span")
            .attr(("class", "tooltip-content"))
            .content(|buf| buf.text("Should be above ").raw(formatter.format(min))),
        ValueState::High { max } => buf
            .node("span")
            .attr(("class", "tooltip-content"))
            .content(|buf| buf.text("Should be below ").raw(formatter.format(max))),
    }
}

fn render_row<'a, W: std::fmt::Write>(
    buf: Buffer<W, Body<'a>>,
    icon: IconKind,
    name: &str,
    formatter: &human_number::Formatter<'_>,
    value: Option<f64>,
    range: (Option<f64>, Option<f64>),
) -> Buffer<W, Body<'a>> {
    let state = ValueState::new(value, range);
    let classname = if state.is_normal() {
        "flex-row mx-md my-sm"
    } else {
        "flex-row mx-md my-sm text-error"
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
                Some(value) => buf
                    .node("div")
                    .cond_attr(!state.is_normal(), ("class", "tooltip"))
                    .content(|buf| {
                        let buf = render_state_icon(buf, state);
                        let buf = buf
                            .node("label")
                            .content(|buf| buf.raw(formatter.format(value)));
                        render_state_tooltip_content(buf, state, formatter)
                    }),
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

impl ValueState {
    fn new(value: Option<f64>, range: (Option<f64>, Option<f64>)) -> Self {
        if let Some(value) = value {
            match range {
                (Some(min), _) if value < min => ValueState::Low { min },
                (_, Some(max)) if value > max => ValueState::High { max },
                _ => ValueState::Normal,
            }
        } else {
            ValueState::Normal
        }
    }

    fn is_normal(&self) -> bool {
        matches!(self, ValueState::Normal)
    }
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
    pub temperature: Option<TimedValue>,
    pub temperature_range: (Option<f64>, Option<f64>),
    pub brightness: Option<TimedValue>,
    pub brightness_range: (Option<f64>, Option<f64>),
    pub moisture: Option<TimedValue>,
    pub moisture_range: (Option<f64>, Option<f64>),
    pub conductivity: Option<TimedValue>,
    pub conductivity_range: (Option<f64>, Option<f64>),
    pub battery: Option<TimedValue>,
    pub battery_range: (Option<f64>, Option<f64>),
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
                    &fmt::TEMPERATURE,
                    self.values.temperature.map(|item| item.value),
                    self.values.temperature_range,
                );
                let buf = render_row(
                    buf,
                    IconKind::Water,
                    "moisture",
                    &fmt::PERCENTAGE,
                    self.values.moisture.as_ref().map(|item| item.value),
                    self.values.moisture_range,
                );
                let buf = render_row(
                    buf,
                    IconKind::Sun,
                    "brightness",
                    &fmt::BRIGHTNESS,
                    self.values.brightness.as_ref().map(|item| item.value),
                    self.values.brightness_range,
                );
                let buf = render_row(
                    buf,
                    IconKind::Dashboard,
                    "conductivity",
                    &fmt::CONDUCTIVITY,
                    self.values.conductivity.as_ref().map(|item| item.value),
                    self.values.conductivity_range,
                );
                let buf = render_row(
                    buf,
                    IconKind::Battery,
                    "battery",
                    &fmt::PERCENTAGE,
                    self.values.battery.as_ref().map(|item| item.value),
                    self.values.battery_range,
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
