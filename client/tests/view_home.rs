use chezmoi_client::component::any_card::AnyCard;
use chezmoi_client::component::bluetooth_devices_card::BluetoothDevicesCard;
use chezmoi_client::component::cpu_card::CpuCard;
use chezmoi_client::component::memory_card::MemoryCard;
use chezmoi_client::component::miflora_card::{LastValues, MifloraCard};
use chezmoi_client::component::swap_card::SwapCard;
use chezmoi_client::view::home::{Section, View};

mod helper;

#[test]
fn with_miflora_cards() {
    helper::write(
        "with-miflora-cards.html",
        View::new(helper::STYLE_PATH)
            .with_section(
                Section::new("With last values")
                    .with_card(AnyCard::Miflora(MifloraCard::new(
                        "00:00:00:00:00",
                        Some("Orchidee"),
                        Some(LastValues {
                            timestamp: 0,
                            temperature: 12.34,
                            brightness: 12.34,
                            moisture: 12.34,
                            conductivity: 12.34,
                            battery: 85.0,
                        }),
                    )))
                    .with_card(AnyCard::Miflora(MifloraCard::new(
                        "00:00:00:00:00",
                        None::<&'static str>,
                        Some(LastValues {
                            timestamp: 0,
                            temperature: 12.34,
                            brightness: 12.34,
                            moisture: 12.34,
                            conductivity: 12.34,
                            battery: 85.0,
                        }),
                    ))),
            )
            .with_section(
                Section::new("Without last values")
                    .with_card(AnyCard::Miflora(MifloraCard::new(
                        "00:00:00:00:00",
                        Some("With name"),
                        None,
                    )))
                    .with_card(AnyCard::Miflora(MifloraCard::new(
                        "00:00:00:00:00",
                        None::<&'static str>,
                        None,
                    ))),
            ),
    );
}

#[test]
fn with_bluetooth_devices() {
    helper::write(
        "with-bluetooth-devices-cards.html",
        View::new(helper::STYLE_PATH)
            .with_section(
                Section::new("No devices").with_card(AnyCard::BluetoothDevices(
                    BluetoothDevicesCard::new([].into_iter()),
                )),
            )
            .with_section(
                Section::new("Many devices").with_card(AnyCard::BluetoothDevices(
                    BluetoothDevicesCard::new(
                        [
                            ("Baz".into(), 10.0),
                            ("Foo".into(), 80.0),
                            ("Hello".into(), 100.0),
                            ("World".into(), 90.0),
                            ("Asterix".into(), 83.0),
                            ("Obelix".into(), 83.0),
                            ("Panoramix".into(), 56.0),
                            ("Bar".into(), 70.0),
                        ]
                        .into_iter(),
                    ),
                )),
            ),
    );
}

#[test]
fn with_system() {
    helper::write(
        "with-system-cards.html",
        View::new(helper::STYLE_PATH).with_section(
            Section::new("Simple")
                .with_card(AnyCard::Memory(MemoryCard::new(
                    1024.0 * 1024.0 * 1024.0 * 64.0,
                    1024.0 * 1024.0 * 1024.0 * 2.2,
                )))
                .with_card(AnyCard::Swap(SwapCard::new(
                    1024.0 * 1024.0 * 1024.0 * 64.0,
                    1024.0 * 1024.0 * 1024.0 * 2.2,
                )))
                .with_card(AnyCard::Cpu(CpuCard::new(68.0))),
        ),
    );
}
