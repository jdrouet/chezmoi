use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

use crate::component::card::Card;
use crate::component::prelude::Component;
use crate::component::{header, page};

#[derive(Debug)]
pub struct SectionProps<'a> {
    pub title: &'a str,
    pub cards: Vec<Card<'a>>,
}

#[derive(Debug, Default)]
pub struct DashboardProps<'a> {
    pub sections: Vec<SectionProps<'a>>,
}

#[derive(Debug, Default)]
pub struct DashboardView;

impl DashboardView {
    fn render_section<'a, W>(
        &self,
        buf: Buffer<W, Body<'a>>,
        props: &SectionProps,
    ) -> Buffer<W, Body<'a>>
    where
        W: WriterExt,
    {
        buf.node("h4")
            .content(|buf| buf.text(props.title))
            .node("section")
            .attr(("class", "dashboard-grid"))
            .content(|buf| props.cards.iter().fold(buf, |buf, card| card.render(buf)))
    }

    fn render_body<'a, W>(
        &self,
        buf: Buffer<W, Body<'a>>,
        props: &DashboardProps,
    ) -> Buffer<W, Body<'a>>
    where
        W: WriterExt,
    {
        let buf = header::render(buf);
        buf.node("main")
            .attr(("class", "container pad-md"))
            .content(|buf| {
                props
                    .sections
                    .iter()
                    .fold(buf, |buf, section| self.render_section(buf, section))
            })
    }

    pub fn render<'a>(&self, props: &DashboardProps<'a>) -> String {
        page::html(another_html_builder::Buffer::default(), |buf| {
            page::head(buf, "Dashboard")
                .node("body")
                .content(|buf| self.render_body(buf, props))
        })
        .into_inner()
    }
}
