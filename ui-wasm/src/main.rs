mod component;
mod view;

use yew::prelude::*;

use crate::component::header::Header;
use crate::view::dashboard::Dashboard;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Header />
            <Dashboard />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
