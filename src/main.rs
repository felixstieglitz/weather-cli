use clap::Parser;

mod cli;
mod api;
mod errors;

fn main() {
    let args = cli::Args::parse();

    println!("{} - {} - {}", args.city, args.country, args.unit);
}
