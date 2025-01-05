use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};
use chezmoi_web_prelude::component::{card_title, value_cell};
use chezmoi_web_prelude::helper::concat::Concat;
use chezmoi_web_prelude::helper::format;
use chezmoi_web_prelude::helper::value::TimedValue;
use chezmoi_web_prelude::prelude::RenderComponent;

use crate::web::Definition;

#[derive(Clone, Copy, Debug)]
pub struct Values {
    pub temperature: Option<TimedValue>,
    pub humidity: Option<TimedValue>,
    pub battery: Option<TimedValue>,
}

#[derive(Clone, Copy, Debug)]
pub struct Component<'a> {
    pub definition: &'a Definition,
    pub values: Values,
    pub with_link: bool,
}

impl Component<'_> {
    fn render_body<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>> {
        buf.node("div")
            .attr(("class", "flex-row flex-grow"))
            .content(|buf| {
                buf.render_component(value_cell::Component {
                    label: "Temperature",
                    formatter: &format::TEMPERATURE,
                    definition: &self.definition.temperature,
                    value: self.values.temperature.as_ref(),
                })
                .render_component(value_cell::Component {
                    label: "Humidity",
                    formatter: &format::PERCENTAGE,
                    definition: &self.definition.humidity,
                    value: self.values.humidity.as_ref(),
                })
                .render_component(value_cell::Component {
                    label: "Battery",
                    formatter: &format::PERCENTAGE,
                    definition: &self.definition.battery,
                    value: self.values.battery.as_ref(),
                })
            })
            .render_component(card_title::Component {
                address: &self.definition.address,
                name: self.definition.name.as_deref(),
            })
    }
}

impl chezmoi_web_prelude::prelude::Component for Component<'_> {
    fn render<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>> {
        let buf = if self.with_link {
            buf.node("a").attr((
                "href",
                Concat("/xiaomi-atc/", self.definition.address.as_str()),
            ))
        } else {
            buf.node("div")
        };
        buf.attr(("class", "atc-sensor card flex-col colspan-3"))
            .content(|buf| self.render_body(buf))
    }
}
