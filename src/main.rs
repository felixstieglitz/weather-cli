use clap::Parser;

mod cli;
mod api;
mod errors;

fn main() {
    let args = cli::Cli::parse();

    println!("{} - {} - {}", args.get_city(), args.get_nation(), args.get_unit());

    let (lat,long):(f32,f32)  = api::get_coord(args.get_city(),args.get_nation()).unwrap();
    println!("{:?}", (lat,long));

    let weather = api::get_weather(lat, long).unwrap();
    println!("{} - {}", weather.get_temperature(), weather.weather_description());

}
