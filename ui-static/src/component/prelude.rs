use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

use crate::context::Context;

pub trait Component {
    fn render<'a, W: WriterExt>(
        &self,
        buf: Buffer<W, Body<'a>>,
        ctx: &Context,
    ) -> Buffer<W, Body<'a>>;
}
