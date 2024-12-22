use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

use crate::component::range::Range;
use crate::component::value::TimedValue;
use crate::component::value_cell;
use crate::helper::format::{BRIGHTNESS, CONDUCTIVITY, PERCENTAGE, TEMPERATURE};

// Available values
// - temperature
// - brightness
// - conductivity
// - moisture
// - battery

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Definition {
    #[serde(default)]
    pub name: Option<String>,
    pub address: String,
    #[serde(default)]
    pub temperature: Range,
    #[serde(default)]
    pub brightness: Range,
    #[serde(default)]
    pub conductivity: Range,
    #[serde(default)]
    pub moisture: Range,
    #[serde(default)]
    pub battery: Range,
}

#[derive(Clone, Debug)]
pub struct Values {
    pub temperature: Option<TimedValue>,
    pub brightness: Option<TimedValue>,
    pub conductivity: Option<TimedValue>,
    pub moisture: Option<TimedValue>,
    pub battery: Option<TimedValue>,
}

#[derive(Debug)]
pub struct MifloraSensorCard<'a> {
    pub definition: &'a Definition,
    pub values: Values,
}

impl crate::component::prelude::Component for MifloraSensorCard<'_> {
    fn render<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>> {
        buf.node("div")
            .attr(("class", "miflora-sensor card flex-col colspan-3 rowspan-2"))
            .content(|buf| {
                buf.node("div")
                    .attr(("class", "content"))
                    .content(|buf| {
                        let buf = value_cell::ValueCell {
                            label: "Temperature",
                            formatter: &TEMPERATURE,
                            definition: &self.definition.temperature,
                            value: self.values.temperature.as_ref(),
                        }
                        .render(buf);
                        let buf = buf
                            .node("div")
                            .attr(("class", "image text-center align-content-center pad-md"))
                            .content(|buf| buf.raw("🪴"));
                        let buf = value_cell::ValueCell {
                            label: "Brightness",
                            formatter: &BRIGHTNESS,
                            definition: &self.definition.brightness,
                            value: self.values.brightness.as_ref(),
                        }
                        .render(buf);
                        let buf = value_cell::ValueCell {
                            label: "Conductivity",
                            formatter: &CONDUCTIVITY,
                            definition: &self.definition.conductivity,
                            value: self.values.conductivity.as_ref(),
                        }
                        .render(buf);
                        let buf = value_cell::ValueCell {
                            label: "Moisture",
                            formatter: &PERCENTAGE,
                            definition: &self.definition.moisture,
                            value: self.values.moisture.as_ref(),
                        }
                        .render(buf);
                        let buf = value_cell::ValueCell {
                            label: "Battery",
                            formatter: &PERCENTAGE,
                            definition: &self.definition.battery,
                            value: self.values.battery.as_ref(),
                        }
                        .render(buf);
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
            })
    }
}
