use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

pub fn render<'a, W, F>(buf: Buffer<W, Body<'a>>, title: &str, children: F) -> Buffer<W, Body<'a>>
where
    F: FnOnce(Buffer<W, Body>) -> Buffer<W, Body>,
    W: WriterExt,
{
    buf.node("h4")
        .content(|buf| buf.text(title))
        .node("section")
        .attr(("class", "dashboard-grid"))
        .content(children)
}
