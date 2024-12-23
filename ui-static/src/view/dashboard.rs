use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

use super::prelude::View;
use crate::component::card::Card;
use crate::component::prelude::Component;
use crate::component::{header, page};
use crate::context::Context;

#[derive(Debug)]
pub struct Section<'a> {
    pub title: &'a str,
    pub cards: Vec<Card<'a>>,
}

impl<'a> Section<'a> {
    pub fn new(title: &'a str, cards: Vec<Card<'a>>) -> Self {
        Self { title, cards }
    }
}

impl Section<'_> {
    fn render<'a, W>(&self, buf: Buffer<W, Body<'a>>, ctx: &Context) -> Buffer<W, Body<'a>>
    where
        W: WriterExt,
    {
        buf.node("h4")
            .content(|buf| buf.text(self.title))
            .node("section")
            .attr(("class", "dashboard-grid"))
            .content(|buf| {
                self.cards
                    .iter()
                    .fold(buf, |buf, card| card.render(buf, ctx))
            })
    }
}

#[derive(Debug, Default)]
pub struct DashboardView<'a> {
    pub base_url: &'a str,
    pub sections: Vec<Section<'a>>,
}

impl<'a> DashboardView<'a> {
    pub fn new(base_url: &'static str, sections: Vec<Section<'a>>) -> Self {
        Self { base_url, sections }
    }
}

impl DashboardView<'_> {
    fn render_body<'a, W>(&self, buf: Buffer<W, Body<'a>>, ctx: &Context) -> Buffer<W, Body<'a>>
    where
        W: WriterExt,
    {
        let buf = header::render(buf);
        buf.node("main")
            .attr(("class", "container pad-md"))
            .content(|buf| {
                self.sections
                    .iter()
                    .fold(buf, |buf, section| section.render(buf, ctx))
            })
    }
}
impl View for DashboardView<'_> {
    fn render(&self, ctx: &Context) -> String {
        page::html(another_html_builder::Buffer::default(), |buf| {
            page::Head::new(
                "Dashboard",
                &[
                    "assets/style-atc-sensor.css",
                    "assets/style-miflora-sensor.css",
                    "assets/style-line-chart.css",
                ],
            )
            .render(buf, ctx)
            .node("body")
            .content(|buf| self.render_body(buf, ctx))
        })
        .into_inner()
    }
}
