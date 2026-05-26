use clap::Parser;

mod cli;
mod api;
mod errors;

fn main() {
    let args = cli::Cli::parse();

    println!("{} - {} - {}", args.get_city(), args.get_nation(), args.get_unit());
    
    let x = api::get_coord(args.get_city(),args.get_nation());
    println!("{:?}", x);
}
