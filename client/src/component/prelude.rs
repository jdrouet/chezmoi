use another_html_builder::{Body, Buffer};

pub(crate) trait Component {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>>;
}
