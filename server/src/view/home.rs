use std::collections::HashSet;
use std::sync::Arc;

use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};
use chezmoi_web_prelude::{
    component::{head, header, html},
    prelude::RenderComponent,
};

use crate::helper::{QueryCollector, QueryResult};

#[derive(Clone, Debug)]
pub struct Config(pub Arc<crate::config::RootConfig>);

impl Config {
    pub fn collect(&self) -> QueryCollector {
        QueryCollector {
            history: Default::default(),
            latest: HashSet::from_iter(
                self.0
                    .xiaomi_atc
                    .iter()
                    .flat_map(|item| item.headers.list())
                    .chain(
                        self.0
                            .xiaomi_miflora
                            .iter()
                            .flat_map(|item| item.headers.list()),
                    ),
            ),
        }
    }

    pub fn build<'a>(&'a self, res: QueryResult) -> Component<'a> {
        Component {
            config: self.0.as_ref(),
            values: res,
        }
    }
}

fn section<'a, W, F>(buf: Buffer<W, Body<'a>>, title: &str, children: F) -> Buffer<W, Body<'a>>
where
    F: FnOnce(Buffer<W, Body>) -> Buffer<W, Body>,
    W: WriterExt,
{
    buf.node("h4")
        .content(|buf| buf.text(title))
        .node("section")
        .attr(("class", "dashboard-grid"))
        .content(children)
}

pub struct Component<'a> {
    config: &'a crate::config::RootConfig,
    values: QueryResult,
}

impl Component<'_> {
    fn render_xiaomi_atc<'a, W>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>>
    where
        W: WriterExt,
    {
        if self.config.xiaomi_atc.is_empty() {
            buf
        } else {
            section(buf, "Thermometers", |buf| {
                self.config.xiaomi_atc.iter().fold(buf, |buf, config| {
                    buf.render_component(chezmoi_sensor_xiaomi_atc::web::card::Component {
                        definition: &config.definition,
                        values: chezmoi_sensor_xiaomi_atc::web::card::Values {
                            temperature: self.values.latest.find(&config.headers.temperature),
                            humidity: self.values.latest.find(&config.headers.humidity),
                            battery: self.values.latest.find(&config.headers.battery),
                        },
                        with_link: true,
                    })
                })
            })
        }
    }

    fn render_xiaomi_miflora<'a, W>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>>
    where
        W: WriterExt,
    {
        if self.config.xiaomi_miflora.is_empty() {
            buf
        } else {
            section(buf, "Plants", |buf| {
                self.config.xiaomi_miflora.iter().fold(buf, |buf, config| {
                    buf.render_component(chezmoi_sensor_xiaomi_miflora::web::card::Component {
                        definition: &config.definition,
                        values: chezmoi_sensor_xiaomi_miflora::web::card::Values {
                            temperature: self.values.latest.find(&config.headers.temperature),
                            brightness: self.values.latest.find(&config.headers.brightness),
                            conductivity: self.values.latest.find(&config.headers.conductivity),
                            moisture: self.values.latest.find(&config.headers.moisture),
                            battery: self.values.latest.find(&config.headers.battery),
                        },
                        with_link: true,
                    })
                })
            })
        }
    }
}

impl<'a> Component<'a> {
    pub fn render(&self) -> String {
        html::html(Default::default(), |buf| {
            buf.render_component(head::Component::new(
                "Home",
                &[
                    chezmoi_web_prelude::asset::STYLE_ATC_SENSOR_CSS_PATH,
                    chezmoi_web_prelude::asset::STYLE_MIFLORA_SENSOR_CSS_PATH,
                    chezmoi_web_prelude::asset::STYLE_LINE_CHART_CSS_PATH,
                ],
            ))
            .node("body")
            .content(|buf| {
                buf.render_component(header::Component::new("Home"))
                    .node("main")
                    .attr(("class", "container pad-md flex-grow scroll-y"))
                    .content(|buf| {
                        let buf = self.render_xiaomi_atc(buf);
                        let buf = self.render_xiaomi_miflora(buf);
                        buf
                    })
            })
        })
        .into_inner()
    }
}
