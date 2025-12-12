use leptos::prelude::*;

use crate::CurrentWeather;
use crate::view::get_icon_from_condition;

#[component]
pub fn CurrentWeather(current_weather: RwSignal<CurrentWeather>) -> impl IntoView {

    let current_temp = move || current_weather.read().temp;
    let current_feels_like = move || current_weather.read().feels_like;
    let current_condition = move || current_weather.read().condition.clone();
    let icon = get_icon_from_condition(current_condition());

    view! {
        <div class="cell is-col-start-2 is-row-span-2 is-row-start-2">
            <div class="box glass" style="width: 200px; height: 160px; position: relative;">
                <div class="icon is-large is-white" style="position: absolute; top: 15px; right: 15px;">{icon}</div>
                <div style="position: absolute; bottom: 15px; left: 15px;">
                    <p class="title is-3 mb-2 has-text-white"> {move || format!("{:.0}°C", current_temp())} </p>
                    <p class="subtitle is-7 has-text-white"> "Feels like: "{move || format!("{:.0}°C", current_feels_like())}</p>
                </div>
            </div>
        </div>
    }
}