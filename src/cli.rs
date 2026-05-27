use std::fmt::{Display};
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long)]
    city: String,
    #[clap(short, long)]
    nation: String,
    #[clap(short, long)]
    unit: Unit,
}
impl Cli {
    pub fn get_city(&self) -> String {
        self.city.clone()
    }
    pub fn get_nation(&self) -> String {
        self.nation.clone()
    }
    pub fn get_unit(&self) -> Unit {
        self.unit.clone()
    }
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Unit {
    Celsius,
    Fahrenheit,
}
impl Unit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Unit::Celsius => "celsius",
            Unit::Fahrenheit => "fahrenheit",
        }
    }
}
impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}