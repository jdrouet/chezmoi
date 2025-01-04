use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};
use chezmoi_web_prelude::component::card_title;
use chezmoi_web_prelude::component::value_cell;
use chezmoi_web_prelude::helper::concat::Concat;
use chezmoi_web_prelude::helper::format;
use chezmoi_web_prelude::helper::range::Range;
use chezmoi_web_prelude::helper::value::TimedValue;
use chezmoi_web_prelude::prelude::RenderComponent;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Definition {
    #[serde(default)]
    pub name: Option<String>,
    pub address: String,
    #[serde(default)]
    pub temperature: Range<f64>,
    #[serde(default)]
    pub brightness: Range<f64>,
    #[serde(default)]
    pub conductivity: Range<f64>,
    #[serde(default)]
    pub moisture: Range<f64>,
    #[serde(default)]
    pub battery: Range<f64>,
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
pub struct Component<'a> {
    pub definition: &'a Definition,
    pub values: Values,
    pub with_link: bool,
}

impl Component<'_> {
    fn render_body<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>> {
        buf.node("div")
            .attr(("class", "content"))
            .content(|buf| {
                buf.render_component(value_cell::Component {
                    label: "Temperature",
                    formatter: &format::TEMPERATURE,
                    definition: &self.definition.temperature,
                    value: self.values.temperature.as_ref(),
                })
                .node("div")
                .attr(("class", "image text-center align-content-center pad-md"))
                .content(|buf| buf.raw("🪴"))
                .render_component(value_cell::Component {
                    label: "Brightness",
                    formatter: &format::BRIGHTNESS,
                    definition: &self.definition.brightness,
                    value: self.values.brightness.as_ref(),
                })
                .render_component(value_cell::Component {
                    label: "Conductivity",
                    formatter: &format::CONDUCTIVITY,
                    definition: &self.definition.conductivity,
                    value: self.values.conductivity.as_ref(),
                })
                .render_component(value_cell::Component {
                    label: "Moisture",
                    formatter: &format::PERCENTAGE,
                    definition: &self.definition.moisture,
                    value: self.values.moisture.as_ref(),
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
                Concat("/miflora-sensor/", self.definition.address.as_str()),
            ))
        } else {
            buf.node("div")
        };
        buf.attr(("class", "miflora-sensor card flex-col colspan-3"))
            .content(|buf| self.render_body(buf))
    }
}
