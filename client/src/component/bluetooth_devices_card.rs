use std::borrow::Cow;

use another_html_builder::{Body, Buffer};

#[derive(Debug)]
pub struct LastValues {
    pub timestamp: u64,
    pub temperature: f64,
    pub brightness: f64,
    pub moisture: f64,
    pub conductivity: f64,
    pub battery: f64,
}

#[derive(Debug)]
pub struct BluetoothDevicesCard {
    devices: Vec<(Cow<'static, str>, f64)>,
    power_formatter: human_number::Formatter<'static>,
}

impl BluetoothDevicesCard {
    pub fn new(devices: impl Iterator<Item = (Cow<'static, str>, f64)>) -> Self {
        let mut devices = Vec::from_iter(devices);
        devices.sort_by(|first, second| second.1.total_cmp(&first.1));
        Self {
            devices,
            power_formatter: human_number::Formatter::si(),
        }
    }

    fn render_device_row<'v, W: std::fmt::Write>(
        &self,
        buf: Buffer<W, Body<'v>>,
        (name, power): &(Cow<'static, str>, f64),
    ) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr(("class", "flex-row m-sm mx-md"))
            .content(|buf| {
                buf.node("div")
                    .attr(("class", "flex-1"))
                    .content(|buf| buf.text(name.as_ref()))
                    .node("div")
                    .content(|buf| buf.raw(self.power_formatter.format(*power)))
                    .node("progress")
                    .attr(("value", *power as u64))
                    .attr(("max", 100))
                    .attr(("min", 0))
                    .content(|buf| buf)
            })
    }

    fn render_device_list<'v, W: std::fmt::Write>(
        &self,
        buf: Buffer<W, Body<'v>>,
    ) -> Buffer<W, Body<'v>> {
        self.devices
            .iter()
            .fold(buf, |buf, item| self.render_device_row(buf, item))
    }
}

impl super::prelude::Component for BluetoothDevicesCard {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr((
                "class",
                "card bluetooth-devices shadow min-w-500px h-150px m-md flex-col",
            ))
            .content(|buf| {
                buf.node("div")
                    .attr(("class", "card-content flex-1 scroll-y"))
                    .content(|buf| self.render_device_list(buf))
                    .node("div")
                    .attr(("class", "card-footer"))
                    .content(|buf| buf.text("Bluetooth devices"))
            })
    }
}
