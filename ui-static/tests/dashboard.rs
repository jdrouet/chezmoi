use std::collections::HashMap;

use chezmoi_ui_static::component::card::{atc_sensor, line_chart, miflora_sensor, Card};
use chezmoi_ui_static::component::range::Range;
use chezmoi_ui_static::component::value::TimedValue;
use chezmoi_ui_static::context::Context;
use chezmoi_ui_static::view::dashboard;
use chezmoi_ui_static::view::prelude::View;

mod helper;

const CTX: Context = Context::relative();

#[test]
fn atc_sensor() {
    let definition = atc_sensor::Definition {
        name: Some("Living room".into()),
        address: "00:00:00:00:00".into(),
        temperature: Range {
            min: Some(19.0),
            max: Some(22.0),
        },
        humidity: Range {
            min: Some(40.0),
            max: Some(70.0),
        },
        battery: Range {
            min: Some(10.0),
            max: None,
        },
    };
    let view = dashboard::DashboardView {
        base_url: "",
        sections: vec![dashboard::Section {
            title: "Home",
            cards: vec![Card::AtcSensor(atc_sensor::AtcSensorCard {
                definition: &definition,
                values: atc_sensor::Values {
                    temperature: Some(TimedValue {
                        timestamp: 0,
                        value: 18.5,
                    }),
                    humidity: Some(TimedValue {
                        timestamp: 0,
                        value: 82.5,
                    }),
                    battery: Some(TimedValue {
                        timestamp: 0,
                        value: 90.0,
                    }),
                },
            })],
        }],
    };
    let view = view.render(&CTX);
    helper::write("dashboard-atc-sensor.html", view);
}

#[test]
fn miflora_sensor() {
    let definition = miflora_sensor::Definition {
        name: None,
        address: "00:00:00:00:00:00".into(),
        temperature: Range::default(),
        brightness: Range::default(),
        conductivity: Range::default(),
        moisture: Range::default(),
        battery: Range::default(),
    };
    let view = dashboard::DashboardView {
        base_url: "",
        sections: vec![dashboard::Section {
            title: "Home",
            cards: vec![Card::MifloraSensor(miflora_sensor::MifloraSensorCard {
                definition: &definition,
                values: miflora_sensor::Values {
                    temperature: None,
                    brightness: None,
                    conductivity: None,
                    moisture: None,
                    battery: None,
                },
            })],
        }],
    };
    let view = view.render(&CTX);
    helper::write("dashboard-miflora-sensor.html", view);
}

#[test]
fn line_chart() {
    let definition = line_chart::Definition {
        title: "Hello World".into(),
        x_range: (0, 60),
        y_range: (0.0, 100.0),
    };
    let view = dashboard::DashboardView {
        base_url: "",
        sections: vec![dashboard::Section {
            title: "Home",
            cards: vec![Card::LineChart(line_chart::LineChartCard {
                definition: &definition,
                values: line_chart::Values {
                    metrics: HashMap::from_iter(
                        [(
                            "foo".into(),
                            vec![
                                TimedValue::new(0, 25.0),
                                TimedValue::new(1, 23.0),
                                TimedValue::new(2, 26.0),
                            ],
                        )]
                        .into_iter(),
                    ),
                },
            })],
        }],
    };
    let view = view.render(&CTX);
    helper::write("dashboard-line-chart.html", view);
}
