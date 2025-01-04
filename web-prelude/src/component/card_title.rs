use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

pub struct Component<'a> {
    pub address: &'a str,
    pub name: Option<&'a str>,
}

impl crate::prelude::Component for Component<'_> {
    fn render<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>> {
        buf.node("div")
            .attr(("class", "card-title border-top"))
            .content(|buf| match self.name {
                Some(ref name) => buf.text(name).raw(" - ").raw(self.address),
                None => buf.raw(self.address),
            })
    }
}
