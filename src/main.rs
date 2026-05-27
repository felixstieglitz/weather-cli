use clap::Parser;

mod cli;
mod api;
mod errors;

fn main() {
    let args = cli::Cli::parse();
    let unit = args.get_unit();
    let city = args.get_city();
    let country = args.get_nation();
    println!("you entered: {} - {} - {}", city, country, unit.clone());

    let (lat,long):(f32,f32)  = api::get_coord(city.clone(), country).unwrap();
    println!("Located at: {} - {}", lat, long);

    let weather = api::get_weather(lat, long, unit.clone()).unwrap();
    println!("It is {} {} and {} in {}", weather.get_temperature(), unit.clone(), weather.weather_description(), city);

}
