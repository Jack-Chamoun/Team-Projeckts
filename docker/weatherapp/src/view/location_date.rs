use leptos::prelude::*;
use crate::view::icons::{Calendar, Location};
use chrono::Local;

#[component]
pub fn LocationDate(city: RwSignal<String>, country: RwSignal<String>) -> impl IntoView {

    let current_date = RwSignal::new(
        Local::now().format("%A, %d %B %Y").to_string()
    );

    view! {
        <div class="cell is-col-start-2 is-col-span-2 is-flex is-gap-4">
            <div class="icon-text is-gap-1">
                <span class="icon">
                    <Location />
                </span>
                <span>{city}", " {country}</span>
            </div>
            <div class="icon-text is-gap-1">
                <span class="icon">
                    <Calendar />
                </span>
                <span>{current_date}</span>
            </div>
        </div>
    }
}