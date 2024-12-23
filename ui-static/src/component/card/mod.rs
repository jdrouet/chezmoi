use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

use crate::context::Context;

pub mod atc_sensor;
pub mod line_chart;
pub mod miflora_sensor;

#[derive(Debug)]
pub enum Card<'a> {
    AtcSensor(atc_sensor::AtcSensorCard<'a>),
    LineChart(line_chart::LineChartCard<'a>),
    MifloraSensor(miflora_sensor::MifloraSensorCard<'a>),
}

impl crate::component::prelude::Component for Card<'_> {
    fn render<'a, W: WriterExt>(
        &self,
        buf: Buffer<W, Body<'a>>,
        ctx: &Context,
    ) -> Buffer<W, Body<'a>> {
        match self {
            Self::AtcSensor(inner) => inner.render(buf, ctx),
            Self::LineChart(inner) => inner.render(buf, ctx),
            Self::MifloraSensor(inner) => inner.render(buf, ctx),
        }
    }
}
