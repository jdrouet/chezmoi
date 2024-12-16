use yew::prelude::*;

use crate::component::atc_sensor_card::{AtcSensorCard, ValueProps};

#[function_component]
pub fn Dashboard() -> Html {
    html! {
        <main class="container pad-md">
            <h4>{"Main"}</h4>
            <section class="dashboard-grid">
                <AtcSensorCard
                    title="Living room"
                    temperature={ValueProps {
                        min_value: Some(18.0),
                        max_value: Some(22.0),
                        value: Some(23.0),
                        timestamp: None,
                    }}
                    humidity={ValueProps {
                        min_value: Some(40.0),
                        max_value: Some(60.0),
                        value: Some(65.0),
                        timestamp: None,
                    }}
                    battery={ValueProps {
                        min_value: Some(10.0),
                        max_value: None,
                        value: Some(65.0),
                        timestamp: None,
                    }}
                />
            </section>
        </main>
    }
}
