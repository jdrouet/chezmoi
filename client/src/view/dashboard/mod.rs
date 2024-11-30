use std::borrow::Cow;

use another_html_builder::{Body, Buffer};

use crate::component::card::AnyCard;
use crate::component::prelude::Component;

#[derive(Debug)]
pub struct Section<'a> {
    name: Cow<'a, str>,
    cards: Vec<AnyCard<'a>>,
}

impl<'a> Section<'a> {
    pub fn new<N: Into<Cow<'a, str>>>(name: N) -> Self {
        Self {
            name: name.into(),
            cards: Vec::default(),
        }
    }
}

impl<'a> Section<'a> {
    pub fn add_card(&mut self, card: AnyCard<'a>) {
        self.cards.push(card);
    }

    pub fn with_card(mut self, card: AnyCard<'a>) -> Self {
        self.cards.push(card);
        self
    }

    pub fn maybe_with_card(mut self, card: Option<AnyCard<'a>>) -> Self {
        if let Some(inner) = card {
            self.cards.push(inner);
        }
        self
    }
}

impl<'a> Component for Section<'a> {
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

#[derive(Debug, Default)]
pub struct View<'a> {
    sections: Vec<Section<'a>>,
}

impl<'a> View<'a> {
    pub fn new(sections: Vec<Section<'a>>) -> Self {
        Self { sections }
    }

    pub fn with_section(mut self, section: Section<'a>) -> Self {
        self.sections.push(section);
        self
    }
}

impl<'a> View<'a> {
    #[inline]
    fn render_head<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        crate::component::head::Head::new("Home").render(buf)
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

impl<'a> super::prelude::View for View<'a> {
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
