use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <main>
            <p>{"Hello World"}</p>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
