use std::borrow::Cow;

use another_html_builder::{Body, Buffer};

pub(crate) struct Header {
    title: Cow<'static, str>,
}

impl Header {
    pub fn new(title: impl Into<Cow<'static, str>>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl super::prelude::Component for Header {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("header").content(|buf| {
            buf.node("section")
                .content(|buf| buf.text(self.title.as_ref()))
        })
    }
}
