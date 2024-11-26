#[derive(Debug)]
pub enum AnyCard {
    BluetoothDevices(super::bluetooth_devices_card::BluetoothDevicesCard),
    Cpu(super::cpu_card::CpuCard),
    Memory(super::memory_card::MemoryCard),
    Miflora(super::miflora_card::MifloraCard),
    Swap(super::swap_card::SwapCard),
}

impl super::prelude::Component for AnyCard {
    fn render<'v, W: std::fmt::Write>(
        &self,
        buf: another_html_builder::Buffer<W, another_html_builder::Body<'v>>,
    ) -> another_html_builder::Buffer<W, another_html_builder::Body<'v>> {
        match self {
            Self::BluetoothDevices(inner) => inner.render(buf),
            Self::Cpu(inner) => inner.render(buf),
            Self::Memory(inner) => inner.render(buf),
            Self::Miflora(inner) => inner.render(buf),
            Self::Swap(inner) => inner.render(buf),
        }
    }
}
