use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

pub trait Component {
    fn render<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>>;
}
