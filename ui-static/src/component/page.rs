use another_html_builder::attribute::AttributeValue;
use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

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

pub fn head<'a, W>(buf: Buffer<W, Body<'a>>, title: &str, base_url: &str) -> Buffer<W, Body<'a>>
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
            .attr(("href", Concat(base_url, "assets/style.css")))
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
