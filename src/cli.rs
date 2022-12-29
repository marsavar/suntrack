use chrono::NaiveDate;
use clap::Parser;

#[derive(Parser)]
/// A command line tool that lets you know when the sun rises and sets in a city
/// on any given date
pub struct Cli {
    /// The name of the city
    pub city: String,
    /// Use local time instead of UTC
    #[clap(short, long)]
    pub local: bool,
    /// Show times for a given date, which must be provided in the format YYYY-MM-DD.
    /// Defaults to today if not provided
    #[clap(short, long)]
    pub date: Option<NaiveDate>,
}
