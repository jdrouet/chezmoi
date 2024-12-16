use yew::prelude::*;

use crate::component::value_cell::ValueCell;

#[derive(Default, Properties, PartialEq)]
pub struct ValueProps {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub value: Option<f64>,
    pub timestamp: Option<u64>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: &'static str,
    pub temperature: ValueProps,
    pub humidity: ValueProps,
    pub battery: ValueProps,
}

#[function_component]
pub fn AtcSensorCard(props: &Props) -> Html {
    html! {
        <div class="card flex-col colspan-3">
            <div class="flex-row flex-grow">
                <ValueCell
                    label="Temperature"
                    formatter={crate::helper::TEMPERATURE_UNIT.clone()}
                    min_value={props.temperature.min_value}
                    max_value={props.temperature.max_value}
                    value={props.temperature.value}
                    timestamp={props.temperature.timestamp}
                />
                <ValueCell
                    label="Humidity"
                    formatter={crate::helper::PERCENTAGE_UNIT.clone()}
                    min_value={props.humidity.min_value}
                    max_value={props.humidity.max_value}
                    value={props.humidity.value}
                    timestamp={props.humidity.timestamp}
                />
                <ValueCell
                    label="Battery"
                    formatter={crate::helper::PERCENTAGE_UNIT.clone()}
                    min_value={props.battery.min_value}
                    max_value={props.battery.max_value}
                    value={props.battery.value}
                    timestamp={props.battery.timestamp}
                />
            </div>
            <div class="card-title border-top">
                {props.title}
            </div>
        </div>
    }
}
