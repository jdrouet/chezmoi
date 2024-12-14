use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component]
pub fn Header(_props: &Props) -> Html {
    html! {
        <header>
            <div class="container flex-row space-between padx-md">
                <a class="text-bold text-lg text-nodeco" href="/">{"Chezmoi"}</a>
            </div>
        </header>
    }
}
