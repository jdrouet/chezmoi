use human_number::Formatter;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: &'static str,
    pub formatter: Formatter<'static>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub value: Option<f64>,
    pub timestamp: Option<u64>,
}

impl Props {
    fn value_color(&self) -> &'static str {
        match (self.min_value, self.value, self.max_value) {
            (Some(min), Some(value), _) if min > value => "text-danger",
            (_, Some(value), Some(max)) if max < value => "text-danger",
            _ => "text-default",
        }
    }

    fn arrow(&self) -> Option<&'static str> {
        match (self.min_value, self.value, self.max_value) {
            (Some(min), Some(value), _) if min > value => Some("🔺"),
            (_, Some(value), Some(max)) if max < value => Some("🔻"),
            _ => None,
        }
    }
}

#[function_component]
pub fn ValueCell(props: &Props) -> Html {
    html! {
        <div class="flex-grow text-center align-content-center pad-md separated">
            <p class={format!("text-bold text-xl mb-md {}", props.value_color())}>
                {props.arrow().map(|i| html! { <span class="icon mr-sm">{i}</span>})}
                {props.value.map(|v| props.formatter.format(v).to_string()).unwrap_or_else(|| String::from("-"))}
            </p>
            <p class="text-xs">{props.label}</p>
        </div>
    }
}
