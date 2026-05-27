use reqwest;
use serde::Deserialize;
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
    let city = results.into_iter()
        .find(|c| c.name == name && c.country == country)
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
    pub fn weather_description(&self) -> &'static str {
        match self.weathercode {
            0 => "Klar",
            1..=3 => "Bewölkt",
            45 | 48 => "Nebel",
            51..=55 => "Nieselregen",
            61..=65 => "Regen",
            71..=75 => "Schnee",
            80..=82 => "Regenschauer",
            95..=99 => "Gewitter",
            _ => "Unbekannt",
        }
    }
}

pub fn get_weather(latitude: f32, longitude: f32) -> Result<CurrentWeather, AppError> {
    let request_format = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current_weather=true",
    );
    let weather_response: WeatherResponse = reqwest::blocking::get(request_format)?.json()?;
    Ok(weather_response.current_weather)
}
