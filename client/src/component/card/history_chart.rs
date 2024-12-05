use std::ops::Range;

use another_html_builder::{Body, Buffer};

use crate::component::helper::Classnames;
use crate::component::line_chart::{LineChart, Serie};
use crate::size::{Dimension, Size};

#[derive(Debug)]
pub struct Card<'a> {
    title: &'a str,
    dimension: Dimension,
    content: LineChart<'a>,
}

impl<'a> Card<'a> {
    pub fn new(
        title: &'a str,
        dimension: Dimension,
        series: Vec<Serie<'a>>,
        x_range: Range<u64>,
    ) -> Self {
        let (size_x, margin_left) = match dimension.width {
            Size::Sm => (190, 25),
            Size::Md => (410, 35),
        };
        let (size_y, margin_bottom) = match dimension.height {
            Size::Sm => (120, 10),
            Size::Md => (280, 15),
        };
        Self {
            title,
            dimension,
            content: LineChart::new((size_x, size_y), margin_left, margin_bottom, series)
                .with_x_range(x_range),
        }
    }
}

impl<'a> crate::component::prelude::Component for Card<'a> {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr((
                "class",
                Classnames::from(("card shadow m-md flex-col", Some(self.dimension))),
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
