use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

pub struct Properties<'a> {
    pub title: &'a str,
}

pub struct Component<'a> {
    title: &'a str,
}

impl<'a> Component<'a> {
    pub const fn new(title: &'a str) -> Self {
        Self { title }
    }
}

impl<'a> crate::prelude::Buildable<Properties<'a>> for Component<'a> {
    fn build(props: &Properties<'a>) -> Self {
        Self { title: props.title }
    }
}

impl<'a> crate::prelude::Component for Component<'a> {
    fn render<'w, W: WriterExt>(&self, buf: Buffer<W, Body<'w>>) -> Buffer<W, Body<'w>> {
        buf.node("header").content(|buf| {
            buf.node("div")
                .attr(("class", "container flex-row space-between padx-md"))
                .content(|buf| {
                    buf.node("a")
                        .attr(("class", "text-bold text-lg text-nodeco"))
                        .attr(("href", "/"))
                        .content(|buf| buf.text(self.title))
                })
        })
    }
}
