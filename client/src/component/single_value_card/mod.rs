use std::borrow::Cow;

use another_html_builder::{Body, Buffer};
use human_number::Formatter;

use crate::helper::Classnames;

pub(crate) struct SingleValueCard {
    classname: Option<Cow<'static, str>>,
    title: Option<Cow<'static, str>>,
    value: f64,
    formatter: Formatter<'static>,
}

impl SingleValueCard {
    pub fn new(
        classname: Option<Cow<'static, str>>,
        title: Option<Cow<'static, str>>,
        value: f64,
        formatter: Formatter<'static>,
    ) -> Self {
        Self {
            classname,
            title,
            value,
            formatter,
        }
    }
}

impl super::prelude::Component for SingleValueCard {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr((
                "class",
                Classnames::new(
                    "card shadow m-md flex-col flex-1 min-h-150px min-w-250px",
                    self.classname.as_deref(),
                ),
            ))
            .content(|buf| {
                buf.node("div")
                    .attr((
                        "class",
                        "single-value-card-content flex-1 text-center bg-success",
                    ))
                    .content(|buf| buf.raw(self.formatter.format(self.value)))
                    .optional(self.title.as_deref(), |buf, title| {
                        buf.node("div")
                            .attr(("class", "card-footer text-center"))
                            .content(|buf| buf.text(title))
                    })
            })
    }
}
