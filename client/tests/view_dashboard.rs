use chezmoi_client::component::card::AnyCard;
use chezmoi_client::view::dashboard::{Section, View};

mod helper;

const FAKE_ADDRESS: &str = "00:00:00:00:00";

#[test]
fn with_atc_thermometer() {
    use chezmoi_client::component::card::atc_thermometer::{Card, Values};

    helper::write(
        "with-atc-thermometer-cards.html",
        View::default()
            .with_window(chezmoi_client::view::dashboard::TimePickerDuration::OneWeek)
            .with_section(
                Section::new("With values").with_card(AnyCard::AtcThermometer(Card::new(
                    FAKE_ADDRESS,
                    None,
                    Values {
                        timestamp: Some(0),
                        temperature: Some(12.34),
                        humidity: Some(12.34),
                        battery: Some(12.34),
                    },
                ))),
            ),
    );
}

#[test]
fn with_bluetooth_devices() {
    use chezmoi_client::component::card::bluetooth_devices::{Card, DeviceValues};

    helper::write(
        "with-bluetooth-devices-cards.html",
        View::default()
            .with_section(
                Section::new("No devices")
                    .with_card(AnyCard::BluetoothDevices(Card::new(Vec::new()))),
            )
            .with_section(
                Section::new("Many devices").with_card(AnyCard::BluetoothDevices(Card::new(vec![
                    DeviceValues {
                        address: "00:00:00:00:00",
                        name: Some("Foo"),
                        tx_power: 80.0,
                        battery: None,
                        timestamp: 0,
                    },
                    DeviceValues {
                        address: "00:00:00:00:01",
                        name: Some("Baz"),
                        tx_power: 10.0,
                        battery: None,
                        timestamp: 0,
                    },
                    DeviceValues {
                        address: "00:00:00:00:02",
                        name: Some("Hello"),
                        tx_power: 100.0,
                        battery: None,
                        timestamp: 0,
                    },
                    DeviceValues {
                        address: "00:00:00:00:03",
                        name: Some("World"),
                        tx_power: 90.0,
                        battery: None,
                        timestamp: 0,
                    },
                    DeviceValues {
                        address: "00:00:00:00:04",
                        name: Some("Asterix"),
                        tx_power: 83.0,
                        battery: None,
                        timestamp: 0,
                    },
                    DeviceValues {
                        address: "00:00:00:00:05",
                        name: Some("Obelix"),
                        tx_power: 83.0,
                        battery: None,
                        timestamp: 0,
                    },
                    DeviceValues {
                        address: "00:00:00:00:06",
                        name: Some("Panoramix"),
                        tx_power: 56.0,
                        battery: None,
                        timestamp: 0,
                    },
                    DeviceValues {
                        address: "00:00:00:00:07",
                        name: Some("Bar"),
                        tx_power: 70.0,
                        battery: None,
                        timestamp: 0,
                    },
                ]))),
            ),
    );
}

#[test]
fn with_system() {
    use chezmoi_client::component::card::{system_cpu, system_memory, system_swap, AnyCard};

    helper::write(
        "with-system-cards.html",
        View::default().with_section(
            Section::new("Simple")
                .with_card(AnyCard::Memory(system_memory::Card::new(
                    Some(1024.0 * 1024.0 * 1024.0 * 64.0),
                    Some(1024.0 * 1024.0 * 1024.0 * 2.2),
                )))
                .with_card(AnyCard::Swap(system_swap::Card::new(
                    Some(1024.0 * 1024.0 * 1024.0 * 64.0),
                    Some(1024.0 * 1024.0 * 1024.0 * 2.2),
                )))
                .with_card(AnyCard::Cpu(system_cpu::Card::new(Some(68.0))))
                .with_card(AnyCard::Cpu(system_cpu::Card::new(None))),
        ),
    );
}

#[test]
fn with_miflora_cards() {
    use chezmoi_client::component::card::miflora::{Card, TimedValue, ValueState, Values};

    let flower_url = "https://www.withouraloha.com/wp-content/uploads/2018/01/orchid-care.jpg";

    helper::write(
        "with-miflora-cards.html",
        View::default()
            .with_section(
                Section::new("With last values")
                    .with_card(AnyCard::Miflora(Card::new(
                        "00:00:00:00:00",
                        Some("Orchidee"),
                        None,
                        Values {
                            temperature: Some(TimedValue::from((0, 12.34, ValueState::Normal))),
                            brightness: Some(TimedValue::from((0, 12.34, ValueState::Normal))),
                            moisture: Some(TimedValue::from((0, 12.34, ValueState::Normal))),
                            conductivity: Some(TimedValue::from((0, 12.34, ValueState::Normal))),
                            battery: Some(TimedValue::from((0, 12.34, ValueState::Normal))),
                        },
                    )))
                    .with_card(AnyCard::Miflora(Card::new(
                        "00:00:00:00:00",
                        None::<&'static str>,
                        Some(flower_url),
                        Values {
                            temperature: Some(TimedValue::from((0, 12.34, ValueState::Normal))),
                            brightness: Some(TimedValue::from((
                                0,
                                12.34,
                                ValueState::Low { min: 15.0 },
                            ))),
                            moisture: Some(TimedValue::from((0, 12.34, ValueState::Normal))),
                            conductivity: Some(TimedValue::from((
                                0,
                                12.34,
                                ValueState::High { max: 10.0 },
                            ))),
                            battery: Some(TimedValue::from((0, 12.34, ValueState::Normal))),
                        },
                    ))),
            )
            .with_section(
                Section::new("Without last values")
                    .with_card(AnyCard::Miflora(Card::new(
                        "00:00:00:00:00",
                        Some("With name"),
                        None,
                        Values {
                            temperature: None,
                            brightness: None,
                            moisture: None,
                            conductivity: None,
                            battery: None,
                        },
                    )))
                    .with_card(AnyCard::Miflora(Card::new(
                        "00:00:00:00:00",
                        Some("With name"),
                        Some(flower_url),
                        Values {
                            temperature: Some(TimedValue::from((0, 12.34, ValueState::Normal))),
                            brightness: Some(TimedValue::from((0, 12.34, ValueState::Normal))),
                            moisture: None,
                            conductivity: None,
                            battery: None,
                        },
                    )))
                    .with_card(AnyCard::Miflora(Card::new(
                        "00:00:00:00:00",
                        None::<&'static str>,
                        None,
                        Values {
                            temperature: None,
                            brightness: None,
                            moisture: None,
                            conductivity: None,
                            battery: None,
                        },
                    ))),
            ),
    );
}
