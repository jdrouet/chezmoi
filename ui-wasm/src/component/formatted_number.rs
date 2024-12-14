use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub formatter: human_number::Formatter<'static>,
    pub value: f64,
}

#[function_component]
pub fn FormattedNumber(props: &Props) -> Html {
    html! {
        <span>
            {props.formatter.format(props.value).to_string()}
        </span>
    }
}
