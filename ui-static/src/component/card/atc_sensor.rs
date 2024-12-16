use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

use crate::component::range::Range;
use crate::component::value_cell;
use crate::helper::format::{PERCENTAGE, TEMPERATURE};

#[derive(Debug)]
pub struct AtcSensorCard<'a> {
    pub name: Option<&'a str>,
    pub address: String,
    pub temperature_definition: Range,
    pub temperature: Option<value_cell::Value>,
    pub humidity_definition: Range,
    pub humidity: Option<value_cell::Value>,
    pub battery_definition: Range,
    pub battery: Option<value_cell::Value>,
}

impl crate::component::prelude::Component for AtcSensorCard<'_> {
    fn render<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>> {
        buf.node("div")
            .attr(("class", "card flex-col colspan-3"))
            .content(|buf| {
                buf.node("div")
                    .attr(("class", "flex-row flex-grow"))
                    .content(|buf| {
                        let buf = value_cell::ValueCell {
                            label: "Temperature",
                            formatter: &TEMPERATURE,
                            definition: &self.temperature_definition,
                            value: self.temperature.as_ref(),
                        }
                        .render(buf);
                        let buf = value_cell::ValueCell {
                            label: "Humidity",
                            formatter: &PERCENTAGE,
                            definition: &self.humidity_definition,
                            value: self.humidity.as_ref(),
                        }
                        .render(buf);
                        let buf = value_cell::ValueCell {
                            label: "Battery",
                            formatter: &PERCENTAGE,
                            definition: &self.battery_definition,
                            value: self.battery.as_ref(),
                        }
                        .render(buf);
                        buf
                    })
                    .node("div")
                    .attr(("class", "card-title border-top"))
                    .content(|buf| match self.name {
                        Some(name) => buf.text(name).raw(" - ").raw(self.address.as_str()),
                        None => buf.raw(self.address.as_str()),
                    })
            })
    }
}
