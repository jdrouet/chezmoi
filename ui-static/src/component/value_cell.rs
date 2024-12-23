use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};
use human_number::Formatter;

use super::range::Range;
use super::value::TimedValue;
use crate::context::Context;
use crate::helper::classes::Cn;

#[allow(unused)]
enum Status {
    TooLow { min: f64 },
    TooHight { max: f64 },
    Normal,
}

impl Status {
    fn color(&self) -> &'static str {
        match self {
            Self::TooLow { .. } | Self::TooHight { .. } => "text-danger",
            _ => "text-default",
        }
    }

    fn icon(&self) -> Option<&'static str> {
        match self {
            Self::TooLow { .. } => Some("🔺"),
            Self::TooHight { .. } => Some("🔻"),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ValueCell<'a> {
    pub label: &'a str,
    pub formatter: &'a Formatter<'static>,
    pub definition: &'a Range,
    pub value: Option<&'a TimedValue>,
}

impl ValueCell<'_> {
    fn status(&self) -> Status {
        match (self.definition.min, self.value, self.definition.max) {
            (Some(min), Some(value), _) if min > value.value => Status::TooLow { min },
            (_, Some(value), Some(max)) if max < value.value => Status::TooHight { max },
            _ => Status::Normal,
        }
    }
}

impl crate::component::prelude::Component for ValueCell<'_> {
    fn render<'a, W: WriterExt>(
        &self,
        buf: Buffer<W, Body<'a>>,
        _ctx: &Context,
    ) -> Buffer<W, Body<'a>> {
        buf.node("div")
            .attr((
                "class",
                "value-cell text-center align-content-center pad-md",
            ))
            .content(|buf| {
                let status = self.status();
                buf.node("p")
                    .attr(("class", Cn("text-bold text-xl mb-md", status.color())))
                    .content(|buf| {
                        let buf = buf.optional(status.icon(), |buf, icon| {
                            buf.node("span")
                                .attr(("class", "icon mr-sm"))
                                .content(|buf| buf.raw(icon))
                        });
                        match self.value {
                            Some(value) => buf.raw(self.formatter.format(value.value)),
                            None => buf.text("-"),
                        }
                    })
                    .node("p")
                    .attr(("class", "text-xs"))
                    .content(|buf| buf.text(self.label))
            })
    }
}
