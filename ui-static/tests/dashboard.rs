use chezmoi_ui_static::component::card::atc_sensor::AtcSensorCard;
use chezmoi_ui_static::component::card::Card;
use chezmoi_ui_static::component::range::Range;
use chezmoi_ui_static::component::value_cell;
use chezmoi_ui_static::view::dashboard;

mod helper;

#[test]
fn simple() {
    let view = dashboard::DashboardView {
        base_url: "",
        sections: vec![dashboard::Section {
            title: "Home",
            cards: vec![Card::AtcSensor(AtcSensorCard {
                name: Some("Living room"),
                address: "00:00:00:00:00".into(),
                temperature_definition: Range {
                    min: Some(19.0),
                    max: Some(22.0),
                },
                temperature: Some(value_cell::Value {
                    value: 18.5,
                    timestamp: 0,
                }),
                humidity_definition: Range {
                    min: Some(40.0),
                    max: Some(70.0),
                },
                humidity: Some(value_cell::Value {
                    value: 82.5,
                    timestamp: 0,
                }),
                battery_definition: Range {
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
    let view = view.render();
    helper::write("dashboard-simple.html", view);
}
