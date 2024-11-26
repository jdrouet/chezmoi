use std::borrow::Cow;

use another_html_builder::{Body, Buffer};

use crate::component::any_card::AnyCard;
use crate::component::prelude::Component;

#[derive(Debug)]
pub struct Section {
    name: Cow<'static, str>,
    cards: Vec<AnyCard>,
}

impl Section {
    pub fn new<N: Into<Cow<'static, str>>>(name: N) -> Self {
        Self {
            name: name.into(),
            cards: Vec::default(),
        }
    }
}

impl Section {
    pub fn with_card(mut self, card: AnyCard) -> Self {
        self.cards.push(card);
        self
    }
}

impl Component for Section {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("section").content(|buf| {
            buf.node("h3")
                .attr(("class", "mt-xl"))
                .content(|buf| buf.text(self.name.as_ref()))
                .node("div")
                .attr(("class", "flex-row flex-wrap"))
                .content(|buf| self.cards.iter().fold(buf, |buf, card| card.render(buf)))
        })
    }
}

#[derive(Debug)]
pub struct View {
    sections: Vec<Section>,
    style_path: &'static str,
}

impl View {
    pub fn new(style_path: &'static str) -> Self {
        Self {
            sections: Default::default(),
            style_path,
        }
    }

    pub fn with_section(mut self, section: Section) -> Self {
        self.sections.push(section);
        self
    }
}

impl View {
    #[inline]
    fn render_head<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        crate::component::head::Head::new("Home", self.style_path).render(buf)
    }

    fn render_content<'v, W: std::fmt::Write>(
        &self,
        buf: Buffer<W, Body<'v>>,
    ) -> Buffer<W, Body<'v>> {
        buf.node("main").content(|buf| {
            self.sections
                .iter()
                .fold(buf, |buf, section| section.render(buf))
        })
    }

    fn render_body<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("body").content(|buf| {
            let buf = crate::component::header::Header::new("Home").render(buf);
            self.render_content(buf)
        })
    }
}

impl super::prelude::View for View {
    fn render(self) -> String {
        another_html_builder::Buffer::default()
            .doctype()
            .node("html")
            .attr(("lang", "en"))
            .content(|buf| {
                let buf = self.render_head(buf);
                let buf = self.render_body(buf);
                buf
            })
            .into_inner()
    }
}
