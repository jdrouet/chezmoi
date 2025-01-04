use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

use crate::prelude::RenderComponent;

struct LinkStyleSheet<'a> {
    href: &'a str,
}

impl crate::prelude::Component for LinkStyleSheet<'_> {
    fn render<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>> {
        buf.node("link")
            .attr(("rel", "stylesheet"))
            .attr(("href", self.href))
            .close()
    }
}

pub struct Component<'a> {
    title: &'a str,
    styles: &'a [&'a str],
}

impl<'a> Component<'a> {
    pub const fn new(title: &'a str, styles: &'a [&'a str]) -> Self {
        Self { title, styles }
    }
}

impl crate::prelude::Component for Component<'_> {
    fn render<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>> {
        buf.node("head").content(|buf| {
            let buf = buf
                .node("meta")
                .attr(("charset", "utf-8"))
                .close()
                .node("meta")
                .attr(("name", "viewport"))
                .attr(("content", "width=device-width"))
                .close()
                .node("title")
                .content(|buf| buf.text(self.title))
                .render_component(LinkStyleSheet {
                    href: "https://cdn.jsdelivr.net/npm/galmuri/dist/galmuri.css",
                })
                .render_component(LinkStyleSheet {
                    href: crate::asset::STYLE_GLOBAL_CSS_PATH,
                });
            self.styles.iter().fold(buf, |buf, s| {
                buf.render_component(LinkStyleSheet { href: *s })
            })
        })
    }
}
