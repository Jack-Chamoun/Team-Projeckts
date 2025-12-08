use leptos::prelude::*;
use crate::view::{get_icon_from_condition, icons::*};
use crate::ThreeHoursForecast;

#[component]
pub fn ThreeHoursForecast(three_hours_forecast: RwSignal<Vec<ThreeHoursForecast>>, city_name: RwSignal<String>) -> impl IntoView {  
       
    view! {
        <div class="cell is-col-start-1 is-row-span-4 is-row-span-7">
            <div class="box glass">
                <article class="media">
                    <div class="icon-text is-gap-1">
                        <Clock />
                        <p>3-HOURS-FORECAST</p>
                    </div>
                </article>

                <For
                    each={move || three_hours_forecast.get()} 
                    key={move |day| format!("{}-{:?}", city_name.get(), day.clone())}
                    children={move |day| {
                        let hour = day.hour.clone();
                        let condition = day.condition.clone();
                        let temperature = day.temperature.clone();
                        
                        let icon = get_icon_from_condition(condition);

                        view! {
                            <article class="media is-align-items-center mb-0 py-1">
                                <div class="media-content has-text-right" style="width: 6ch;">
                                    <span class="time">{format!("{:02}", hour)}</span>
                                </div>
                                <div class="media-content has-text-centered" style="width: 6rem;">
                                    <div class="icon is-medium">{icon}</div>
                                </div>
                                <div class="media-content has-text-left" style="width: 6ch;">
                                    <span class="temperature">{format!("{:.0}°C", temperature)}</span>
                                </div>
                            </article>
                        }

                    }}
                />
            </div>
        </div>

    }
}