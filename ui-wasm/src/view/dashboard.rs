use yew::prelude::*;

#[function_component]
pub fn Dashboard() -> Html {
    html! {
        <main class="container pad-md dashboard-grid">
            <div class="card colspan-2 rowspan-2">
                <div class="card-title">{"First"}</div>
            </div>
            <div class="card"><div class="card-title">{"Second"}</div></div>
            <div class="card"><div class="card-title">{"Third"}</div></div>
            <div class="card"><div class="card-title">{"Fourth"}</div></div>
        </main>
    }
}
