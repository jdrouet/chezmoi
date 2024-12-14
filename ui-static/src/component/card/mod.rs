use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

pub mod atc_sensor;

#[derive(Debug)]
pub enum Card<'a> {
    AtcSensor(atc_sensor::AtcSensorCard<'a>),
}

impl crate::component::prelude::Component for Card<'_> {
    fn render<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>> {
        match self {
            Self::AtcSensor(inner) => inner.render(buf),
        }
    }
}
