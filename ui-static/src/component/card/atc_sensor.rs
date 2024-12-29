use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

use crate::component::prelude::Component;
use crate::component::range::Range;
use crate::component::value::TimedValue;
use crate::component::value_cell;
use crate::context::Context;
use crate::helper::attributes::Concat;
use crate::helper::format::{PERCENTAGE, TEMPERATURE};

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Definition {
    #[serde(default)]
    pub name: Option<String>,
    pub address: String,
    #[serde(default)]
    pub temperature: Range<f64>,
    #[serde(default)]
    pub humidity: Range<f64>,
    #[serde(default)]
    pub battery: Range<f64>,
}

#[derive(Debug)]
pub struct Values {
    pub temperature: Option<TimedValue>,
    pub humidity: Option<TimedValue>,
    pub battery: Option<TimedValue>,
}

#[derive(Debug)]
pub struct AtcSensorCard<'a> {
    pub definition: &'a Definition,
    pub values: Values,
    pub with_link: bool,
}

impl AtcSensorCard<'_> {
    fn render_body<'a, W: WriterExt>(
        &self,
        buf: Buffer<W, Body<'a>>,
        ctx: &Context,
    ) -> Buffer<W, Body<'a>> {
        buf.node("div")
            .attr(("class", "flex-row flex-grow"))
            .content(|buf| {
                let buf = value_cell::ValueCell {
                    label: "Temperature",
                    formatter: &TEMPERATURE,
                    definition: &self.definition.temperature,
                    value: self.values.temperature.as_ref(),
                }
                .render(buf, ctx);
                let buf = value_cell::ValueCell {
                    label: "Humidity",
                    formatter: &PERCENTAGE,
                    definition: &self.definition.humidity,
                    value: self.values.humidity.as_ref(),
                }
                .render(buf, ctx);
                let buf = value_cell::ValueCell {
                    label: "Battery",
                    formatter: &PERCENTAGE,
                    definition: &self.definition.battery,
                    value: self.values.battery.as_ref(),
                }
                .render(buf, ctx);
                buf
            })
            .node("div")
            .attr(("class", "card-title border-top"))
            .content(|buf| match self.definition.name {
                Some(ref name) => buf
                    .text(name)
                    .raw(" - ")
                    .raw(self.definition.address.as_str()),
                None => buf.raw(self.definition.address.as_str()),
            })
    }
}

impl Component for AtcSensorCard<'_> {
    fn render<'a, W: WriterExt>(
        &self,
        buf: Buffer<W, Body<'a>>,
        ctx: &Context,
    ) -> Buffer<W, Body<'a>> {
        let buf = if self.with_link {
            buf.node("a").attr((
                "href",
                Concat(ctx.url("atc-sensor/"), self.definition.address.as_str()),
            ))
        } else {
            buf.node("div")
        };
        buf.attr(("class", "atc-sensor card flex-col colspan-3"))
            .content(|buf| self.render_body(buf, ctx))
    }
}
