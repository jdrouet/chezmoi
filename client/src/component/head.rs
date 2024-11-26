use std::borrow::Cow;

use another_html_builder::{Body, Buffer};

pub(crate) struct Head {
    title: Cow<'static, str>,
    style_path: &'static str,
}

impl Head {
    pub fn new(title: impl Into<Cow<'static, str>>, style_path: &'static str) -> Self {
        Self {
            title: title.into(),
            style_path,
        }
    }
}

impl super::prelude::Component for Head {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("head").content(|buf| {
            buf.node("meta")
                .attr(("charset", "utf-8"))
                .close()
                .node("meta")
                .attr(("name", "viewport"))
                .attr(("content", "width=device-width, initial-scale=1"))
                .close()
                .node("title")
                .content(|buf| buf.text("🏠 Chez Moi - ").text(self.title.as_ref()))
                .node("link")
                .attr(("rel", "stylesheet"))
                .attr(("href", self.style_path))
                .close()
        })
    }
}
