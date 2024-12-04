pub mod atc_thermometer;
mod binary_usage;
pub mod bluetooth_devices;
pub(crate) mod container;
pub mod history_chart;
pub mod miflora;
pub mod system_cpu;
pub mod system_memory;
pub mod system_swap;

#[derive(Debug)]
pub enum AnyCard<'a> {
    AtcThermometer(atc_thermometer::Card<'a>),
    BluetoothDevices(bluetooth_devices::Card<'a>),
    Cpu(system_cpu::Card),
    HistoryChart(history_chart::Card<'a>),
    Memory(system_memory::Card),
    Miflora(miflora::Card<'a>),
    Swap(system_swap::Card),
}

impl<'a> super::prelude::Component for AnyCard<'a> {
    fn render<'v, W: std::fmt::Write>(
        &self,
        buf: another_html_builder::Buffer<W, another_html_builder::Body<'v>>,
    ) -> another_html_builder::Buffer<W, another_html_builder::Body<'v>> {
        match self {
            Self::AtcThermometer(inner) => inner.render(buf),
            Self::BluetoothDevices(inner) => inner.render(buf),
            Self::Cpu(inner) => inner.render(buf),
            Self::HistoryChart(inner) => inner.render(buf),
            Self::Memory(inner) => inner.render(buf),
            Self::Miflora(inner) => inner.render(buf),
            Self::Swap(inner) => inner.render(buf),
        }
    }
}
