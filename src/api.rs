use reqwest;
use serde::Deserialize;
use crate::errors::AppError;


#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    results: Option<Vec<LocationResult>>,
}

#[derive(Debug, Deserialize)]
struct LocationResult {
    latitude: f64,
    longitude: f64,
    name: String,
    country: String,
}

pub fn get_coord(name: String, country: String) -> Result<(f64,f64), AppError> {
    let url_city = name.replace(' ', "%20");
    let request_format = format!("https://geocoding-api.open-meteo.com/v1/search?name={url_city}",);

    let response: GeocodingResponse = reqwest::blocking::get(request_format)?.json()?;
    let results = response.results.ok_or(AppError::CityNotFound)?;
    let city = results.into_iter()
        .find(|c| c.name == name && c.country == country)
        .ok_or(AppError::CityNotFound)?;
    Ok((city.latitude, city.longitude))
}