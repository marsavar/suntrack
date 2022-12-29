use crate::cli::Cli;
use crate::models::{City, Coords, Row};
use crate::utils::{city_and_country, pick_option};
use asciifolding::fold_to_ascii;
use clap::Parser;
use models::MapOfCities;
use std::process;

mod cli;
mod models;
mod utils;

fn main() {
    let mut all_cities = MapOfCities::new();

    // A modified version of the simplemaps.com World Cities Database,
    // which is licensed under Creative Commons Attribution 4.0
    // https://simplemaps.com/data/world-cities
    // `city` and `iso2` have been merged into one
    let cities = include_str!("cities.csv");

    let mut reader = csv::ReaderBuilder::new().from_reader(cities.as_bytes());

    for result in reader.deserialize() {
        let row: Row = result.unwrap();
        all_cities.map.insert(
            row.city,
            Coords {
                lat: row.lat,
                long: row.long,
            },
        );
    }

    let cli = Cli::parse();
    let city = cli.city;

    let cities = all_cities
        .map
        .keys()
        .filter(|c| c.starts_with(&fold_to_ascii(&city.trim().to_lowercase())))
        .collect::<Vec<&City>>();

    match cities.len() {
        0 => {
            println!("Found no results for {city}");
            process::exit(0);
        }
        1 => all_cities.print_coords(cities, 0, cli.local, cli.date),
        l => 'prompt_input: loop {
            println!("Found {} options for {}:", l, city);

            for choice in 1..=l {
                let (city, country) = city_and_country(cities[choice - 1].clone());
                println!("{choice}) {} {}", city, country);
            }

            let choice = pick_option();

            if let Ok(c) = choice {
                if c > l {
                    continue 'prompt_input;
                } else {
                    all_cities.print_coords(cities, c - 1, cli.local, cli.date);
                    break 'prompt_input;
                }
            }
        },
    }
}
