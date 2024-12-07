use chezmoi_client::component::card::AnyCard;
use chezmoi_client::view::dashboard::{Section, TimePickerDuration, View};
use chezmoi_client::{Dimension, Size};

mod helper;

const FAKE_ADDRESS: &str = "00:00:00:00:00";

#[test]
fn with_atc_thermometer() {
    use chezmoi_client::component::card::atc_thermometer::{Card, Values};

    helper::write(
        "with-atc-thermometer-cards.html",
        View::new(Vec::new(), TimePickerDuration::OneWeek).with_section(
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
        View::new(Vec::new(), TimePickerDuration::OneWeek)
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
        View::new(Vec::new(), TimePickerDuration::OneWeek).with_section(
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
    use chezmoi_client::component::card::miflora::{Card, TimedValue, Values};

    let flower_url = "https://www.withouraloha.com/wp-content/uploads/2018/01/orchid-care.jpg";

    helper::write(
        "with-miflora-cards.html",
        View::new(Vec::new(), TimePickerDuration::OneWeek)
            .with_section(
                Section::new("With last values")
                    .with_card(AnyCard::Miflora(Card::new(
                        "00:00:00:00:00",
                        Some("Orchidee"),
                        None,
                        Values {
                            temperature: Some(TimedValue::from((0, 12.34))),
                            temperature_range: Default::default(),
                            brightness: Some(TimedValue::from((0, 12.34))),
                            brightness_range: Default::default(),
                            moisture: Some(TimedValue::from((0, 12.34))),
                            moisture_range: Default::default(),
                            conductivity: Some(TimedValue::from((0, 12.34))),
                            conductivity_range: Default::default(),
                            battery: Some(TimedValue::from((0, 12.34))),
                            battery_range: Default::default(),
                        },
                    )))
                    .with_card(AnyCard::Miflora(Card::new(
                        "00:00:00:00:00",
                        None::<&'static str>,
                        Some(flower_url),
                        Values {
                            temperature: Some(TimedValue::from((0, 12.34))),
                            temperature_range: Default::default(),
                            brightness: Some(TimedValue::from((0, 12.34))),
                            brightness_range: (Some(15.0), None),
                            moisture: Some(TimedValue::from((0, 12.34))),
                            moisture_range: (Some(10.0), Some(20.0)),
                            conductivity: Some(TimedValue::from((0, 12.34))),
                            conductivity_range: (Some(5.0), Some(10.0)),
                            battery: Some(TimedValue::from((0, 12.34))),
                            battery_range: (Some(5.0), None),
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
                            temperature_range: (None, None),
                            brightness: None,
                            brightness_range: (None, None),
                            moisture: None,
                            moisture_range: (None, None),
                            conductivity: None,
                            conductivity_range: (None, None),
                            battery: None,
                            battery_range: (None, None),
                        },
                    )))
                    .with_card(AnyCard::Miflora(Card::new(
                        "00:00:00:00:00",
                        Some("With name"),
                        Some(flower_url),
                        Values {
                            temperature: Some(TimedValue::from((0, 12.34))),
                            temperature_range: (None, None),
                            brightness: Some(TimedValue::from((0, 12.34))),
                            brightness_range: (None, None),
                            moisture: None,
                            moisture_range: (None, None),
                            conductivity: None,
                            conductivity_range: (None, None),
                            battery: None,
                            battery_range: (None, None),
                        },
                    )))
                    .with_card(AnyCard::Miflora(Card::new(
                        "00:00:00:00:00",
                        None::<&'static str>,
                        None,
                        Values {
                            temperature: None,
                            temperature_range: (None, None),
                            brightness: None,
                            brightness_range: (None, None),
                            moisture: None,
                            moisture_range: (None, None),
                            conductivity: None,
                            conductivity_range: (None, None),
                            battery: None,
                            battery_range: (None, None),
                        },
                    ))),
            ),
    );
}

#[test]
fn with_history_chart_cards() {
    use chezmoi_client::component::card::history_chart::Card;
    use chezmoi_client::component::line_chart::Serie;

    helper::write(
        "with-history-chart-cards.html",
        View::new(Vec::new(), TimePickerDuration::OneWeek)
            .with_section(
                Section::new("Small height")
                    .with_card(AnyCard::HistoryChart(Card::new(
                        "Small width",
                        Dimension::new(Size::Sm, Size::Sm),
                        vec![Serie::new(
                            "CPU",
                            vec![
                                (0, 10.0),
                                (1, 20.0),
                                (2, 10.0),
                                (5, 70.0),
                                (7, 40.0),
                                (10, 70.0),
                            ],
                        )],
                        Some(0..10),
                        Some(0.0..100.0),
                    )))
                    .with_card(AnyCard::HistoryChart(Card::new(
                        "Medium width",
                        Dimension::new(Size::Md, Size::Sm),
                        vec![Serie::new(
                            "CPU",
                            vec![
                                (0, 10.0),
                                (1, 20.0),
                                (2, 10.0),
                                (5, 70.0),
                                (7, 40.0),
                                (10, 70.0),
                            ],
                        )],
                        Some(0..10),
                        Some(0.0..100.0),
                    ))),
            )
            .with_section(
                Section::new("Medium height")
                    .with_card(AnyCard::HistoryChart(Card::new(
                        "Small width",
                        Dimension::new(Size::Sm, Size::Md),
                        vec![Serie::new(
                            "CPU",
                            vec![
                                (0, 10.0),
                                (1, 20.0),
                                (2, 10.0),
                                (5, 70.0),
                                (7, 40.0),
                                (10, 70.0),
                            ],
                        )],
                        Some(0..10),
                        Some(0.0..100.0),
                    )))
                    .with_card(AnyCard::HistoryChart(Card::new(
                        "CPU",
                        Dimension::new(Size::Md, Size::Md),
                        vec![Serie::new(
                            "Medium width",
                            vec![
                                (0, 10.0),
                                (1, 20.0),
                                (2, 10.0),
                                (5, 70.0),
                                (7, 40.0),
                                (10, 70.0),
                            ],
                        )],
                        Some(0..10),
                        Some(0.0..100.0),
                    ))),
            ),
    );
}
