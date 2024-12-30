use super::prelude::View;
use crate::component::prelude::Component;
use crate::component::{header, page};
use crate::context::Context;

#[derive(Debug, Default)]
pub struct ErrorView<'a> {
    message: &'a str,
}

impl<'a> ErrorView<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}

impl View for ErrorView<'_> {
    fn render(&self, ctx: &Context) -> String {
        page::html(another_html_builder::Buffer::default(), |buf| {
            page::Head::new("Error", &[])
                .render(buf, ctx)
                .node("body")
                .content(|buf| {
                    let buf = header::render(buf);
                    buf.node("main")
                        .attr(("class", "container pad-md"))
                        .content(|buf| {
                            buf.node("div").attr(("class", "card")).content(|buf| {
                                buf.node("div")
                                    .attr(("class", "text-center pad-lg"))
                                    .content(|buf| buf.text(self.message))
                            })
                        })
                })
        })
        .into_inner()
    }
}
