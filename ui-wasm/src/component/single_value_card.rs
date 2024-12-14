use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColorLevel {
    Success,
    Default,
    Danger,
}

impl ColorLevel {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "text-success",
            Self::Default => "text-default",
            Self::Danger => "text-danger",
        }
    }
}

impl std::fmt::Display for ColorLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: &'static str,
    pub level: ColorLevel,
    pub value: String,
}

#[function_component]
pub fn SingleValueCard(props: &Props) -> Html {
    html! {
        <div class="card cell-h-sm flex-col">
            <div class="card-content align-content-center flex-grow pad-lg text-center">
                <p class={format!("text-xxl {}", props.level)}>{props.value.as_str()}</p>
            </div>
            <div class="card-title border-top">
                {props.title}
            </div>
        </div>
    }
}
