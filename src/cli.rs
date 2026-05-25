use std::fmt::{Display};
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(short, long)]
    pub city: String,
    #[clap(short, long)]
    pub country: String,
    #[clap(short, long)]
    pub unit: Unit,

}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Unit {
    Celsius,
    Fahrenheit,
}
impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}