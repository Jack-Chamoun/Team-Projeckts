use leptos::prelude::*;
use std::collections::BTreeMap;
use crate::FiveDaysForecast;
use chrono::{Datelike, Local, NaiveDate, Duration};
use crate::view::icons::Clock;

fn format_date(date_str: &str) -> String {
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let today = Local::now().date_naive();
        let tomorrow = today + Duration::days(1);

        if date == today {
            "Today".to_string()
        } else if date == tomorrow {
            "Tomorrow".to_string()
        } else {
            format!("{} {}", date.day(), date.format("%b"))
        }
    } else {
        date_str.to_string()
    }
}

#[component]
pub fn FiveDaysForecast(five_days_forecast: RwSignal<BTreeMap<String, FiveDaysForecast>>, city_name: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="cell is-col-span-4 is-col-start-2 is-row-start-6 ">
            <div class="box glass">
                <article class="media">
                    <div class="icon-text is-gap-1">
                        <Clock />
                        <span>5-DAYS-FORECAST</span>
                    </div>
                </article>
                
                <div class="media is-flex is-justify-content-space-between">
                    <For
                        each={move || five_days_forecast.get()} 
                        key={move |tempstat| format!("{}-{:?}", city_name.get(), tempstat)}
                        children={move |(_, stats)| {
                            let date = stats.date.clone();
                            let avg = stats.avg.clone();
                            let min = stats.min.clone();
                            let max = stats.max.clone();
                            
                            view! {
                                <div class="media-content">
                                    <p class="has-text-centered mb-2"> {format_date(&date)} </p>
                                    <p class="has-text-centered is-size-4 mb-3">
                                        {format!("{:.0}°C", avg)}
                                    </p>
                                    <div class="columns is-gapless is-size-7">
                                        <div class="column has-text-centered" style="padding-right: 5px; padding-left: 5px;">
                                            <p> {format!("L: {:.0}°C", min)} </p>
                                        </div>
                                        <div class="column has-text-centered" style="padding-right: 5px; padding-left: 5px;">
                                            <p> {format!("H: {:.0}°C", max)} </p>
                                        </div>
                                    </div>
                                </div>
                            }
                        }}
                    />
                </div>

            </div>
        </div>
    }
}