use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

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
