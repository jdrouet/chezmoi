use std::borrow::Cow;

use another_html_builder::{Body, Buffer};

pub(crate) struct Header<Content = ()> {
    title: Cow<'static, str>,
    content: Option<Content>,
}

impl<C> Header<C> {
    pub fn new(title: impl Into<Cow<'static, str>>) -> Self {
        Self {
            title: title.into(),
            content: None,
        }
    }

    pub fn with_content(mut self, content: C) -> Self {
        self.content = Some(content);
        self
    }
}

impl<C: super::prelude::Component> super::prelude::Component for Header<C> {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("header").attr(("class", "shadow")).content(|buf| {
            let buf = buf
                .node("section")
                .content(|buf| buf.text(self.title.as_ref()));
            buf.optional(self.content.as_ref(), |buf, content| {
                buf.node("section").content(|buf| content.render(buf))
            })
        })
    }
}
