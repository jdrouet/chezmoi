use yew::prelude::*;

use crate::component::temperature_card::TemperatureCard;

#[function_component]
pub fn Dashboard() -> Html {
    html! {
        <main class="container pad-md">
            <h4>{"Main"}</h4>
            <section class="dashboard-grid">
                <TemperatureCard title="Living room temperature" value={19.34} min_value={Some(19.5)} max_value={Some(22.0)} />
                <div class="card colspan-all flex-col">
                    <div class="flex-row flex-grow">
                        <div class="flex-grow align-content-center pad-md">
                            {"Hello World"}
                        </div>
                        <div class="flex-grow align-content-center border-left pad-md">
                            {"right"}
                        </div>
                    </div>
                    <div class="card-title border-top">
                        {"Multiple in a row"}
                    </div>
                </div>
                <TemperatureCard title="Living room temperature" value={21.34} min_value={Some(19.5)} max_value={Some(22.0)} />
                <TemperatureCard title="Living room temperature" value={23.34} min_value={Some(19.5)} max_value={Some(22.0)} />
                <div class="card"><div class="card-title">{"Second"}</div></div>
                <div class="card"><div class="card-title">{"Third"}</div></div>
                <div class="card"><div class="card-title">{"Fourth"}</div></div>
            </section>
        </main>
    }
}
