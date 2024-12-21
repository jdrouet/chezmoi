use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

pub mod atc_sensor;
pub mod miflora_sensor;

#[derive(Debug)]
pub enum Card<'a> {
    AtcSensor(atc_sensor::AtcSensorCard<'a>),
    MifloraSensor(miflora_sensor::MifloraSensorCard<'a>),
}

impl crate::component::prelude::Component for Card<'_> {
    fn render<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>> {
        match self {
            Self::AtcSensor(inner) => inner.render(buf),
            Self::MifloraSensor(inner) => inner.render(buf),
        }
    }
}
