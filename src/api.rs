use reqwest;
use serde::Deserialize;
use crate::cli::Unit;
use crate::errors::AppError;


#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    results: Option<Vec<LocationResult>>,
}

#[derive(Debug, Deserialize)]
struct LocationResult {
    latitude: f32,
    longitude: f32,
    name: String,
    country: String,
}

pub fn get_coord(name: String, country: String) -> Result<(f32,f32), AppError> {
    let url_city = name.replace(' ', "%20");
    let request_format = format!("https://geocoding-api.open-meteo.com/v1/search?name={url_city}",);

    let response: GeocodingResponse = reqwest::blocking::get(request_format)?.json()?;
    let results = response.results.ok_or(AppError::CityNotFound)?;
    let name_lower = name.to_lowercase();
    let country_lower = country.to_lowercase();
    let city = results.into_iter()
        .find(|c| c.name.to_lowercase() == name_lower && c.country.to_lowercase() == country_lower)
        .ok_or(AppError::CityNotFound)?;
    Ok((city.latitude, city.longitude))
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    current_weather: CurrentWeather
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    temperature: f32,
    weathercode: u8
}
impl CurrentWeather {
    pub fn get_temperature(&self) -> f32 {
        self.temperature.clone()
    }
    pub fn weather_description(&self) -> &'static str {
        match self.weathercode {
            0 => "Clear",
            1..=3 => "Cloudy",
            45 | 48 => "Fog",
            51..=55 => "Drizzle",
            61..=65 => "Rain",
            71..=75 => "Snow",
            80..=82 => "Rain showers",
            95..=99 => "Thunderstorm",
            _ => "Unknown",
        }
    }
}

pub fn get_weather(latitude: f32, longitude: f32, unit: Unit) -> Result<CurrentWeather, AppError> {
    let request_format = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current_weather=true&temperature_unit={}", unit.as_str(),
    );
    let weather_response: WeatherResponse = reqwest::blocking::get(request_format)?.json()?;
    Ok(weather_response.current_weather)
}
