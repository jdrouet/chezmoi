mod binary_usage;
pub mod bluetooth_devices;
pub mod mi_thermometer;
pub mod miflora;
pub mod system_cpu;
pub mod system_memory;
pub mod system_swap;

#[derive(Debug)]
pub enum AnyCard<'a> {
    BluetoothDevices(bluetooth_devices::Card),
    Cpu(system_cpu::Card),
    Memory(system_memory::Card),
    MiThermometer(mi_thermometer::Card<'a>),
    Miflora(miflora::Card<'a>),
    Swap(system_swap::Card),
}

impl<'a> super::prelude::Component for AnyCard<'a> {
    fn render<'v, W: std::fmt::Write>(
        &self,
        buf: another_html_builder::Buffer<W, another_html_builder::Body<'v>>,
    ) -> another_html_builder::Buffer<W, another_html_builder::Body<'v>> {
        match self {
            Self::BluetoothDevices(inner) => inner.render(buf),
            Self::Cpu(inner) => inner.render(buf),
            Self::Memory(inner) => inner.render(buf),
            Self::MiThermometer(inner) => inner.render(buf),
            Self::Miflora(inner) => inner.render(buf),
            Self::Swap(inner) => inner.render(buf),
        }
    }
}
