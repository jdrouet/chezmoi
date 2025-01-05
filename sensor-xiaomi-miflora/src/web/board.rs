use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};
use chezmoi_web_prelude::component::{card_title, head, header, html, line_chart_card};
use chezmoi_web_prelude::helper::range::Range;
use chezmoi_web_prelude::helper::value::TimedValue;
use chezmoi_web_prelude::prelude::RenderComponent;

use crate::web::Definition;

#[derive(Clone, Debug)]
pub struct Values {
    pub temperature: Vec<TimedValue>,
    pub brightness: Vec<TimedValue>,
    pub conductivity: Vec<TimedValue>,
    pub moisture: Vec<TimedValue>,
    pub battery: Vec<TimedValue>,
}

#[derive(Clone, Debug)]
pub struct Component<'a> {
    pub definition: &'a Definition,
    pub timerange: (u64, u64),
    pub values: Values,
}

impl Component<'_> {
    fn render_body<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>> {
        buf.node("div")
            .attr(("class", "flex-row flex-grow"))
            .content(|buf| {
                buf.render_component(line_chart_card::Component {
                    title: "Temperature",
                    timerange: self.timerange,
                    values: &self.values.temperature,
                    y_range: Range {
                        min: Some(0.0),
                        max: Some(25.0),
                    },
                })
                .render_component(line_chart_card::Component {
                    title: "Brightness",
                    timerange: self.timerange,
                    values: &self.values.brightness,
                    y_range: Range {
                        min: Some(0.0),
                        max: Some(100.0),
                    },
                })
                .render_component(line_chart_card::Component {
                    title: "Conductivity",
                    timerange: self.timerange,
                    values: &self.values.conductivity,
                    y_range: Range {
                        min: Some(0.0),
                        max: Some(100.0),
                    },
                })
                .render_component(line_chart_card::Component {
                    title: "Moisture",
                    timerange: self.timerange,
                    values: &self.values.moisture,
                    y_range: Range {
                        min: Some(0.0),
                        max: Some(100.0),
                    },
                })
                .render_component(line_chart_card::Component {
                    title: "Battery",
                    timerange: self.timerange,
                    values: &self.values.battery,
                    y_range: Range {
                        min: Some(0.0),
                        max: Some(100.0),
                    },
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
        html::html(buf, |buf| {
            buf.render_component(head::Component::new("Plant", &[]))
                .node("body")
                .content(|buf| {
                    buf.render_component(header::Component::new("Plant"))
                        .node("main")
                        .attr(("class", "container pad-md flex-grow scroll-y"))
                        .content(|buf| self.render_body(buf))
                })
        })
    }
}
