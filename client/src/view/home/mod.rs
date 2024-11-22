use another_html_builder::{Body, Buffer};
use human_number::Formatter;

use crate::component::prelude::Component;
use crate::component::single_value_card::SingleValueCard;

#[derive(Debug, Default)]
pub struct View {}

impl View {
    #[inline]
    fn render_head<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        crate::component::head::Head::new("Home").render(buf)
    }

    fn render_content<'v, W: std::fmt::Write>(
        &self,
        buf: Buffer<W, Body<'v>>,
    ) -> Buffer<W, Body<'v>> {
        buf.node("main").content(|buf| {
            buf.node("section")
                .content(|buf| {
                    buf.node("h3")
                        .attr(("class", "mt-xl"))
                        .content(|buf| buf.text("Host"))
                        .node("div")
                        .attr(("class", "flex-row flex-wrap"))
                        .content(|buf| {
                            let buf = SingleValueCard::new(
                                None,
                                Some("CPU temperature".into()),
                                42.0,
                                Formatter::binary().with_unit("B"),
                            )
                            .render(buf);
                            let buf = SingleValueCard::new(
                                None,
                                Some("CPU usage".into()),
                                42.0,
                                Formatter::binary().with_unit("B"),
                            )
                            .render(buf);
                            let buf = SingleValueCard::new(
                                None,
                                Some("Memory usage".into()),
                                42.0,
                                Formatter::binary().with_unit("B"),
                            )
                            .render(buf);
                            buf
                        })
                })
                .node("section")
                .content(|buf| {
                    buf.node("h3")
                        .attr(("class", "mt-xl"))
                        .content(|buf| buf.text("Living room"))
                        .node("div")
                        .attr(("class", "card shadow max-w-400px my-lg"))
                        .content(|buf| buf.text("Hello World!"))
                })
        })
    }

    fn render_body<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("body").content(|buf| {
            let buf = crate::component::header::Header::new("Home").render(buf);
            self.render_content(buf)
        })
    }
}

impl super::prelude::View for View {
    fn render(self) -> String {
        another_html_builder::Buffer::default()
            .doctype()
            .node("html")
            .attr(("lang", "en"))
            .content(|buf| {
                let buf = self.render_head(buf);
                let buf = self.render_body(buf);
                buf
            })
            .into_inner()
    }
}
