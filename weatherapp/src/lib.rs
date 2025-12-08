mod view;
mod api;

use leptos::prelude::*;
use gloo_timers::callback::Interval;
use crate::view::WeatherApp;
use crate::api::fetch_data;
use crate::api::Day;
use api::fetch_esp32_data;
use std::collections::BTreeMap;
use crate::api::fetch_current_data;


#[derive(Clone, Debug)]
pub struct ThreeHoursForecast {
    // Three Hours Forecast
    pub hour: u8,
    pub temperature: f64,
    pub condition: String,
}

#[derive(Clone, Debug)]
pub struct CurrentWeather {
    // Current Weather
    pub temp: f64,
    pub feels_like: f64,
    pub condition: String,
}

#[derive(Clone, Debug)]
pub struct CurrentInformation {
    // Information
    pub wind: f64,
    pub humidity: i32,
    pub condition: String,
    pub visibility: i32,
}

#[derive(Clone, Debug)]
pub struct FiveDaysForecast {
    // Five Days Forecast
    pub date: String,
    pub avg: f64,
    pub min: f64,
    pub max: f64,
}

#[component]
pub fn App() -> impl IntoView {

    let city = RwSignal::new(String::from("Augsburg"));
    let country = RwSignal::new(String::from("Germany"));

    let current_weather_resource = LocalResource::new(move || {
        let city = city.clone();
        async move {
            let city_name = city.get();
            fetch_current_data(city_name).await.ok()
        }
    });

    let weather_resource = LocalResource::new(move || {
        let city = city.clone();
        async move {
            let city_name = city.get();
            fetch_data(city_name).await.ok()
        }
    });

    // ESP32-Project
    let roomtemp = RwSignal::new(None);
    let trigger = RwSignal::new(());

    let esp32_resource = LocalResource::new(move || {
        let _trigger = trigger.get();
        async move {
            fetch_esp32_data().await.ok()
        }
    });

    let three_hours_forecast = RwSignal::new(Vec::<ThreeHoursForecast>::new());
    let current_weather = RwSignal::new(CurrentWeather { temp: 0.0, feels_like: 0.0, condition: String::new()});
    let current_info = RwSignal::new(CurrentInformation { wind: 0.0, humidity: 0, condition: String::new(), visibility: 0 });
    let five_days_forecast = RwSignal::new(BTreeMap::<String, FiveDaysForecast>::new());
    
    let grouped_days = RwSignal::new(BTreeMap::new()); 

    Effect::new(move || {
        match current_weather_resource.get().as_deref() {
            Some(Some(data)) => {
                current_weather.set(
                    CurrentWeather { 
                        temp: data.main.temp, 
                        feels_like: data.main.feels_like, 
                        condition: data.weather.get(0).unwrap().main.clone() 
                    }
                );

                current_info.set(
                    CurrentInformation {
                        wind: data.wind.speed,
                        humidity: data.main.humidity,
                        condition: data.weather.get(0).unwrap().description.clone(),
                        visibility: data.visibility
                    }
                )
            }   
            _ => {}
        };
    });

    Effect::new(move || {
        match weather_resource.get().as_deref() {
            Some(Some(data)) => {

                country.set(data.city.country.clone());

                // Next 9 Entries
                let mut three_hour_entries = Vec::new();
                for entry in data.list.iter().take(9) {
                    let hour: u8 = entry.dt_txt[11..13].parse().unwrap_or(0);
                    let condition = entry.weather.get(0)
                        .map_or("N/A".to_string(), |w| format!("{}", w.main));
                    let temperature = entry.main.temp.clone();

                    three_hour_entries.push(ThreeHoursForecast {
                        hour,
                        condition,
                        temperature,
                    });
                }
                three_hours_forecast.set(three_hour_entries);


                // Grouped Entries by Days (just a helper)
                let mut grouped: BTreeMap<String, Vec<Day>> = BTreeMap::new();
                for day in &data.list {
                    let date = day.dt_txt.split_whitespace().next().unwrap_or("").to_string();
                    grouped.entry(date).or_insert_with(Vec::new).push(day.clone());
                }
                grouped_days.set(grouped.clone());


                // 5 Days Forecast (avg, min, max)
                let mut temp_stats_per_day = BTreeMap::new();

                for (date, entries) in &grouped {
                    let count = entries.len() as f64;
                    let date = date.clone();

                    if count > 0.0 {
                        let sum: f64 = entries.iter().map(|d| d.main.temp).sum();
                        let avg = sum / count;

                        let min = entries.iter().map(|d| d.main.temp_min).fold(f64::INFINITY, f64::min);
                        let max = entries.iter().map(|d| d.main.temp_max).fold(f64::NEG_INFINITY, f64::max);

                        temp_stats_per_day.insert(date.clone(), FiveDaysForecast { date, avg, min, max });
                    }
                }

                five_days_forecast.set(temp_stats_per_day);
            }   
            _ => {}
        };
    });

    // ESP32-Project
    Effect::new(move || {
        let interval = Interval::new(10_000, move || {
            trigger.set(());   
        });
        interval.forget();
    });

    // ESP32-Project
    Effect::new(move || {
        if let Some(Some(data)) = esp32_resource.get().as_deref() {
            roomtemp.set(data.clone().temperature);
        }
    });

    view! {
        <WeatherApp 
            city 
            country
            three_hours_forecast
            current_weather
            roomtemp        // ESP32-Project       
            current_info
            five_days_forecast
        />
    }
}
