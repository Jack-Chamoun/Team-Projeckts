mod three_hours_forecast;
mod current_weather;
mod roomtemperature;
mod current_info;
mod five_days_forecast;
mod icons;
mod description;
mod location_date;
mod search_bar;

use std::collections::BTreeMap;
use icons::*;
use leptos::prelude::*;

use current_info::CurrentInfo;
use description::Description;
use location_date::LocationDate;
use search_bar::SearchBar;
use crate::{CurrentInformation, CurrentWeather, ThreeHoursForecast, FiveDaysForecast};
use three_hours_forecast::ThreeHoursForecast as ThreeHoursForecastComponent;
use five_days_forecast::FiveDaysForecast as FiveDaysForecastComponent;
use current_weather::CurrentWeather as CurrentWeatherComponent;
use roomtemperature::Roomtemperature;

#[component]
pub fn WeatherApp(
    city: RwSignal<String>,
    country: RwSignal<String>,
    three_hours_forecast: RwSignal<Vec<ThreeHoursForecast>>,
    current_weather: RwSignal<CurrentWeather>,
    roomtemp: RwSignal<Option<f64>>,
    current_info: RwSignal<CurrentInformation>,
    five_days_forecast: RwSignal<BTreeMap<String, FiveDaysForecast>>,
) -> impl IntoView {

    view! {
        <div class="container pt-6">
            <div class="fixed-grid has-5-cols">
                <div class="grid">
                    <ThreeHoursForecastComponent three_hours_forecast city_name=city/>
                    <LocationDate city country />
                    <CurrentWeatherComponent current_weather />
                    <Roomtemperature roomtemp />        // ESP32-Project
                    <Description current_weather />
                    <CurrentInfo current_info />
                    <FiveDaysForecastComponent five_days_forecast city_name=city />
                    <SearchBar city />
                </div>
            </div>
        </div>
    }
}

pub fn get_icon_from_condition(condition: String) -> impl IntoView {
     match condition.to_lowercase().as_str() {
        "clear" => {
            view! { <Sunny /> }.into_any()
        }
        "clouds" => {
            view! { <Cloudy /> }.into_any()
        }
        "drizzle" | "rain" => {
            view! { <Rain /> }.into_any()
        }
        "thunderstorm" => {
            view! { <Thunderstorm /> }.into_any()
        } 
        "snow" => {
            view! { <Snow /> }.into_any()
        }
        "mist" => {
            view! { <Mist /> }.into_any()
        }
        "wind" => {
            view! { <Windy /> }.into_any()
        }
        _ => view! { <Sunny /> }.into_any()
    }
}
