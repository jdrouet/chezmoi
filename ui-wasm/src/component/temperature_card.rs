use yew::prelude::*;

use crate::component::single_value_card::{ColorLevel, SingleValueCard};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: &'static str,
    pub value: f64,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
}

#[function_component]
pub fn TemperatureCard(props: &Props) -> Html {
    let level = match (props.min_value, props.max_value) {
        (Some(min), _) if min > props.value => ColorLevel::Danger,
        (_, Some(max)) if max < props.value => ColorLevel::Danger,
        _ => ColorLevel::Default,
    };
    html! {
        <SingleValueCard
            level={level}
            title={props.title}
            value={crate::helper::TEMPERATURE_UNIT.format(props.value).to_string()}
        />
    }
}
