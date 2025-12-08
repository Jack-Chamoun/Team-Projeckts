use leptos::prelude::*;
use crate::CurrentWeather;

#[component]
pub fn Description(current_weather: RwSignal<CurrentWeather>) -> impl IntoView {

    let condition = move || current_weather.read().condition.clone();

    let description = move || {

        match condition().to_lowercase().as_str() {
            "clear" => "The sun's out. Time to fix that Vitamin D deficiency.",
            "clouds" => "Gloomy and cloudy, as usual. Classic German weather.",
            "rain" | "drizzle" => "A little rain never hurt anyone — except everyone's mood.",
            "thunderstorm" => "Someone angered the thunder gods. Again.",
            "snow" => "The slip-n-slide weather is here. It's snowing.",
            "mist" => "Things are getting hazy... literally.",
            "wind" => "Windy enough to challenge your umbrella game.",
            _ => "Sky doing its own mysterious thing again. Proceed with caution.",
        }
    };

    view! {
        <div class="cell is-col-start-2 is-row-start-4 is-col-span-2">
            <div class ="content">
                <span class="description-font">{move || description}</span>
            </div>
        </div>
    }
}

