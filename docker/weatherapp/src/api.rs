use serde::Deserialize;
use std::error::Error;
use gloo_net::http::Request;

#[derive(Deserialize, Debug, Clone)]
pub struct CurrentForecast {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub visibility: i32,
    pub wind: Wind,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Forecast {
    pub city: City,
    pub list: Vec<Day>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Day {
    pub dt_txt: String,
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub humidity: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Weather {
    pub main: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Wind {
    pub speed: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct City {
    pub country: String,
}

// ESP32-Project
#[derive(Deserialize, Debug, Clone)]
pub struct ESP32Data {
    pub temperature: Option<f64>,
}

pub async fn fetch_data(city: String) -> Result<Forecast, Box<dyn Error>> {

    let url = format!("https://api.openweathermap.org/data/2.5/forecast?q={city}&appid=8050aa94a2294f0b93862b769bdbaf8a&units=metric"); 
    let response = Request::get(&url)
        .send()
        .await?;

    if !response.ok() {
        return Err(format!("Error fetching data: {}", response.status()).into());
    }
    
    let data: Forecast = response.json().await?;

    Ok(data)
}

pub async fn fetch_current_data(currentcity: String) -> Result<CurrentForecast, Box<dyn Error>> {

    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={currentcity}&appid=8050aa94a2294f0b93862b769bdbaf8a&units=metric"); 
    let response = Request::get(&url)
        .send()
        .await?;

    if !response.ok() {
        return Err(format!("Error fetching data: {}", response.status()).into());
    }
    
    let data: CurrentForecast = response.json().await?;

    Ok(data)
}

// ESP32-Project
pub async fn fetch_esp32_data() -> Result<ESP32Data, Box<dyn Error>> {

    let ip_address = "roomtemperature.local";

    // TODO: Replace with ESP32 url
    let url = format!("http://{ip_address}/temperature"); 
    let response = Request::get(&url)
        .send()
        .await?;

    if !response.ok() {
        return Err(format!("Error fetching data: {}", response.status()).into());
    }

    let data: ESP32Data = response.json().await?;

    Ok(data)
}
