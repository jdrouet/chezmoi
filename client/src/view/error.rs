use std::borrow::Cow;

use another_html_builder::{Body, Buffer};

use crate::component::prelude::Component;

#[derive(Debug, Default)]
pub struct View {
    message: Cow<'static, str>,
}

impl View {
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }

    #[inline]
    fn render_head<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        crate::component::head::Head::new("Error").render(buf)
    }

    fn render_body<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("body").content(|buf| {
            let buf = crate::component::header::Header::new("Error").render(buf);
            buf.node("main")
                .content(|buf| buf.text(self.message.as_ref()))
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
