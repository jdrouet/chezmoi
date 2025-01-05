use std::collections::HashSet;

use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};
use chezmoi_web_prelude::{
    component::{head, header, html},
    prelude::RenderComponent,
};

use crate::helper::{QueryCollector, QueryResult};

#[derive(Clone, Debug)]
pub struct Config(pub chezmoi_sensor_xiaomi_atc::web::Config);

impl Config {
    pub fn collect(&self) -> QueryCollector {
        QueryCollector {
            history: HashSet::from_iter(self.0.headers.list()),
            latest: HashSet::from_iter(self.0.headers.list()),
        }
    }

    pub fn build<'a>(&'a self, res: QueryResult) -> Component<'a> {
        Component {
            config: &self.0,
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
    config: &'a chezmoi_sensor_xiaomi_atc::web::Config,
    values: QueryResult,
}

impl<'a> Component<'a> {
    pub fn render(&self) -> String {
        html::html(Default::default(), |buf| {
            buf.render_component(head::Component::new("Thermometer", &[]))
                .node("body")
                .content(|buf| {
                    buf.render_component(header::Component::new("Thermometer"))
                        .node("main")
                        .attr(("class", "container pad-md flex-grow scroll-y"))
                        .content(|buf| buf)
                })
        })
        .into_inner()
    }
}
