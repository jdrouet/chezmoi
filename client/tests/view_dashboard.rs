use chezmoi_client::component::card::miflora::ValueState;
use chezmoi_client::component::card::AnyCard;
use chezmoi_client::view::dashboard::{Section, View};

mod helper;

#[test]
fn with_miflora_cards() {
    use chezmoi_client::component::card::miflora::{Card, LastValues, TimedValue};

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
                        LastValues {
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
                        LastValues {
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
                        LastValues {
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
                        LastValues {
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
                        LastValues {
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

#[test]
fn with_bluetooth_devices() {
    use chezmoi_client::component::card::bluetooth_devices::Card;

    helper::write(
        "with-bluetooth-devices-cards.html",
        View::default()
            .with_section(
                Section::new("No devices")
                    .with_card(AnyCard::BluetoothDevices(Card::new([].into_iter()))),
            )
            .with_section(
                Section::new("Many devices").with_card(AnyCard::BluetoothDevices(Card::new(
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
                ))),
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
