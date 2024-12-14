use chezmoi_ui_static::component::card::atc_sensor::AtcSensorCard;
use chezmoi_ui_static::component::card::Card;
use chezmoi_ui_static::component::value_cell;
use chezmoi_ui_static::view::dashboard;

mod helper;

#[test]
fn simple() {
    let props = dashboard::DashboardProps {
        sections: vec![dashboard::SectionProps {
            title: "Home",
            cards: vec![Card::AtcSensor(AtcSensorCard {
                title: "Living room",
                temperature_definition: value_cell::Definition {
                    min: Some(19.0),
                    max: Some(22.0),
                },
                temperature: Some(value_cell::Value {
                    value: 18.5,
                    timestamp: 0,
                }),
                humidity_definition: value_cell::Definition {
                    min: Some(40.0),
                    max: Some(70.0),
                },
                humidity: Some(value_cell::Value {
                    value: 82.5,
                    timestamp: 0,
                }),
                battery_definition: value_cell::Definition {
                    min: Some(10.0),
                    max: None,
                },
                battery: Some(value_cell::Value {
                    value: 21.5,
                    timestamp: 0,
                }),
            })],
        }],
    };
    let view = dashboard::DashboardView::default().render(&props);
    helper::write("dashboard-simple.html", view);
}
