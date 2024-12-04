use another_html_builder::{AttributeValue, Body, Buffer};

use crate::component::helper::Classnames;
use crate::component::line_chart::{LineChart, Serie};

#[derive(Clone, Copy, Debug)]
pub enum CardSize {
    Sm,
    Md,
}

impl CardSize {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CardDimension {
    width: CardSize,
    height: CardSize,
}

impl AttributeValue for CardDimension {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x-{} y-{}", self.width.as_str(), self.height.as_str())
    }
}

#[derive(Debug)]
pub struct Card<'a> {
    title: &'a str,
    dimension: CardDimension,
    content: LineChart<'a>,
}

impl<'a> Card<'a> {
    pub fn new(title: &'a str, width: CardSize, height: CardSize, series: Vec<Serie<'a>>) -> Self {
        let (size_x, margin_left) = match width {
            CardSize::Sm => (190, 25),
            CardSize::Md => (410, 35),
        };
        let (size_y, margin_bottom) = match height {
            CardSize::Sm => (120, 10),
            CardSize::Md => (280, 15),
        };
        Self {
            title,
            dimension: CardDimension { width, height },
            content: LineChart::new((size_x, size_y), margin_left, margin_bottom, series),
        }
    }
}

impl<'a> crate::component::prelude::Component for Card<'a> {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr((
                "class",
                Classnames::from(("card x-sm y-sm shadow m-md flex-col", Some(self.dimension))),
            ))
            .content(|buf| {
                buf.node("div")
                    .attr((
                        "class",
                        "card-content flex-1 text-center align-content-center",
                    ))
                    .content(|buf| self.content.render(buf))
                    .node("div")
                    .attr(("class", "card-footer"))
                    .content(|buf| buf.text(self.title))
            })
    }
}
