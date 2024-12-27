use another_html_builder::attribute::AttributeValue;
use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

use crate::context::Context;

struct Concat<'a>(&'a str, &'a str);

impl AttributeValue for Concat<'_> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

pub fn html<W, F>(buf: Buffer<W, Body<'_>>, children: F) -> Buffer<W, Body<'_>>
where
    F: FnOnce(Buffer<W, Body>) -> Buffer<W, Body>,
    W: WriterExt,
{
    buf.doctype()
        .node("html")
        .attr(("lang", "en"))
        .content(children)
}

pub struct Head {
    title: &'static str,
    styles: &'static [&'static str],
}

impl Head {
    pub const fn new(title: &'static str, styles: &'static [&'static str]) -> Self {
        Self { title, styles }
    }
}

impl crate::component::prelude::Component for Head {
    fn render<'a, W: WriterExt>(
        &self,
        buf: Buffer<W, Body<'a>>,
        ctx: &Context,
    ) -> Buffer<W, Body<'a>> {
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
                .node("link")
                .attr(("rel", "stylesheet"))
                .attr((
                    "href",
                    "https://cdn.jsdelivr.net/npm/galmuri/dist/galmuri.css",
                ))
                .close()
                .node("link")
                .attr(("rel", "stylesheet"))
                .attr(("href", Concat(&ctx.base_url, "assets/style-global.css")))
                .close();
            self.styles.iter().fold(buf, |buf, s| {
                buf.node("link")
                    .attr(("rel", "stylesheet"))
                    .attr(("href", Concat(&ctx.base_url, s)))
                    .close()
            })
        })
    }
}

pub fn head<'a, W>(buf: Buffer<W, Body<'a>>, ctx: &Context, title: &str) -> Buffer<W, Body<'a>>
where
    W: WriterExt,
{
    buf.node("head").content(|buf| {
        buf.node("meta")
            .attr(("charset", "utf-8"))
            .close()
            .node("title")
            .content(|buf| buf.text(title))
            .node("link")
            .attr(("rel", "stylesheet"))
            .attr((
                "href",
                "https://cdn.jsdelivr.net/npm/galmuri/dist/galmuri.css",
            ))
            .close()
            .node("link")
            .attr(("rel", "stylesheet"))
            .attr(("href", Concat(&ctx.base_url, "assets/style-global.css")))
            .close()
            // TODO dynamically load those styles
            .node("link")
            .attr(("rel", "stylesheet"))
            .attr(("href", Concat(&ctx.base_url, "assets/style-atc-sensor.css")))
            .close()
            .node("link")
            .attr(("rel", "stylesheet"))
            .attr((
                "href",
                Concat(&ctx.base_url, "assets/style-miflora-sensor.css"),
            ))
            .close()
    })
}

pub fn body<W, F>(buf: Buffer<W, Body<'_>>, children: F) -> Buffer<W, Body<'_>>
where
    F: FnOnce(Buffer<W, Body>) -> Buffer<W, Body>,
    W: WriterExt,
{
    buf.node("body").content(children)
}

#[inline(always)]
pub fn empty<W>(buf: Buffer<W, Body<'_>>) -> Buffer<W, Body<'_>>
where
    W: WriterExt,
{
    buf
}
