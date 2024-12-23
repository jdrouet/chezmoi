use std::collections::HashMap;

use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

use crate::component::range::Range;
use crate::component::value::TimedValue;
use crate::context::Context;

fn format_hourly(ts: &u64) -> String {
    chrono::DateTime::from_timestamp(*ts as i64, 0)
        .map(|ts| ts.format("%H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Definition {
    pub title: String,
    #[serde(default)]
    pub x_range: Range<u64>,
    #[serde(default)]
    pub y_range: Range<f64>,
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
    fn x_range(&self) -> std::ops::Range<u64> {
        let from = self.definition.x_range.min.unwrap_or(u64::MAX);
        let to = self.definition.x_range.max.unwrap_or(u64::MIN);
        let (from, to) = self.values.iter().fold((from, to), |(from, to), item| {
            (from.min(item.timestamp), to.max(item.timestamp))
        });
        from..to
    }

    fn y_range(&self) -> std::ops::Range<f64> {
        let from = self.definition.y_range.min.unwrap_or(f64::MAX);
        let to = self.definition.y_range.max.unwrap_or(f64::MIN);
        let (from, to) = self.values.iter().fold((from, to), |(from, to), item| {
            (from.min(item.value), to.max(item.value))
        });
        from..to
    }

    fn into_svg(&self, size: (u32, u32)) -> String {
        use plotters::prelude::*;

        // TODO find a way to access the buffer content
        let mut buffer = String::new();
        {
            let root =
                plotters::backend::SVGBackend::with_string(&mut buffer, size).into_drawing_area();
            let mut chart = ChartBuilder::on(&root)
                .margin(10)
                .set_label_area_size(LabelAreaPosition::Left, 30)
                .set_label_area_size(LabelAreaPosition::Bottom, 20)
                .build_cartesian_2d(self.x_range(), self.y_range())
                .unwrap();

            chart
                .configure_mesh()
                .disable_x_mesh()
                .disable_y_mesh()
                .x_labels(30)
                .max_light_lines(4)
                .x_label_formatter(&format_hourly)
                .draw()
                .unwrap();

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
