use std::sync::LazyLock;

use another_html_builder::{Body, Buffer};

static POWER_FORMATTER: LazyLock<human_number::Formatter<'static>> =
    LazyLock::new(human_number::Formatter::si);

#[derive(Debug)]
pub struct DeviceValues<'a> {
    pub address: &'a str,
    pub name: Option<&'a str>,
    pub tx_power: f64,
    pub battery: Option<f64>,
    pub timestamp: u64,
}

#[derive(Debug, Default)]
pub struct Card<'a> {
    devices: Vec<DeviceValues<'a>>,
}

impl<'a> Card<'a> {
    pub fn new(mut devices: Vec<DeviceValues<'a>>) -> Self {
        devices.sort_by(|first, second| second.tx_power.total_cmp(&first.tx_power));
        Self { devices }
    }

    fn render_device_row<'v, W: std::fmt::Write>(
        &self,
        buf: Buffer<W, Body<'v>>,
        device: &DeviceValues<'a>,
    ) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr(("class", "flex-row m-sm mx-md"))
            .content(|buf| {
                buf.node("div")
                    .attr(("class", "flex-1"))
                    .content(|buf| buf.text(device.name.as_deref().unwrap_or(device.address)))
                    .node("div")
                    .content(|buf| buf.raw(POWER_FORMATTER.format(device.tx_power)))
                    .node("progress")
                    .attr(("value", device.tx_power as u64))
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

impl<'a> crate::component::prelude::Component for Card<'a> {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr((
                "class",
                "card bluetooth-devices shadow min-w-500px h-150px m-md flex-col",
            ))
            .content(|buf| {
                buf.node("div")
                    .attr(("class", "card-content flex-1 scroll-y py-md"))
                    .content(|buf| self.render_device_list(buf))
                    .node("div")
                    .attr(("class", "card-footer"))
                    .content(|buf| buf.text("Bluetooth devices"))
            })
    }
}
