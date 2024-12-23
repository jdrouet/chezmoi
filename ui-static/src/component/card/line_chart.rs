use std::collections::HashMap;
use std::ops::Range;

use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

use crate::component::value::TimedValue;
use crate::context::Context;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Definition {
    pub title: String,
    pub x_range: (u64, u64),
    pub y_range: (f64, f64),
}

#[derive(Debug)]
pub struct Values {
    pub metrics: HashMap<String, Vec<TimedValue>>,
}

#[derive(Debug)]
pub struct LineChartCard<'a> {
    pub definition: &'a Definition,
    pub values: Vec<TimedValue>,
}

impl LineChartCard<'_> {
    fn x_range(&self) -> Range<u64> {
        self.definition.x_range.0..self.definition.x_range.1
    }

    fn y_range(&self) -> Range<f64> {
        self.definition.y_range.0..self.definition.y_range.1
    }

    fn into_svg(&self, size: (u32, u32)) -> String {
        use plotters::prelude::*;

        // TODO find a way to access the buffer content
        let mut buffer = String::new();
        {
            let root =
                plotters::backend::SVGBackend::with_string(&mut buffer, size).into_drawing_area();
            // root.fill(&WHITE).unwrap();
            let mut chart = ChartBuilder::on(&root)
                // .margin(10)
                // .set_label_area_size(LabelAreaPosition::Left, self.margin_left)
                // .set_label_area_size(LabelAreaPosition::Bottom, self.margin_bottom)
                .build_cartesian_2d(self.x_range(), self.y_range())
                .unwrap();

            // chart
            //     .configure_mesh()
            //     .disable_x_mesh()
            //     .disable_y_mesh()
            //     .x_labels(30)
            //     .max_light_lines(4)
            //     .x_label_formatter(&format_hourly)
            //     .draw()
            //     .unwrap();

            chart
                .draw_series(
                    LineSeries::new(self.values.iter().map(|p| (p.timestamp, p.value)), &BLUE)
                        .point_size(1),
                )
                .unwrap();
        }

        buffer
    }
}

impl crate::component::prelude::Component for LineChartCard<'_> {
    fn render<'a, W: WriterExt>(
        &self,
        buf: Buffer<W, Body<'a>>,
        _ctx: &Context,
    ) -> Buffer<W, Body<'a>> {
        buf.node("div")
            .attr(("class", "line-chart card flex-col colspan-4"))
            .content(|buf| {
                buf.node("div")
                    .attr(("class", "flex-grow max-sm"))
                    .content(|buf| buf.raw(self.into_svg((600, 400))))
                    .node("div")
                    .attr(("class", "flex-grow min-md max-md"))
                    .content(|buf| buf.raw(self.into_svg((800, 400))))
                    .node("div")
                    .attr(("class", "flex-grow min-lg"))
                    .content(|buf| buf.raw(self.into_svg((1200, 400))))
                    .node("div")
                    .attr(("class", "card-title border-top"))
                    .content(|buf| buf.text(&self.definition.title))
            })
    }
}
