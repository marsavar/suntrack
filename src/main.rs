use crate::cli::Cli;
use crate::io::pick_option;
use crate::models::{City, Coords, Row, Uppercase};
use asciifolding::fold_to_ascii;
use clap::Parser;
use models::MapOfCities;
use regex::Regex;
use std::process;

mod cli;
mod io;
mod models;

fn main() {
    let mut all_cities = MapOfCities::new();

    let cities = include_str!("cities.csv");

    let mut reader = csv::ReaderBuilder::new().from_reader(cities.as_bytes());

    for result in reader.deserialize() {
        let row: Row = result.unwrap();
        all_cities.map.insert(
            format!("{} ({})", row.city, row.iso),
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
        1 => all_cities.print_coords(cities, 0, cli.local),
        l => 'prompt_input: loop {
            println!("Found {} options for {}:", l, city);

            for choice in 1..=l {
                let regex = Regex::new(r"(.+) (\(.+\))").unwrap();
                let captures = &regex.captures(&cities[choice - 1]).unwrap();

                let (city, country) = (
                    &captures[1].to_string().to_uppercase_every_word(),
                    &captures[2].to_uppercase(),
                );

                println!("{choice}) {} {}", city, country);
            }

            let choice = pick_option();

            if let Ok(c) = choice {
                if c > l {
                    continue 'prompt_input;
                } else {
                    all_cities.print_coords(cities, c - 1, cli.local);
                    break 'prompt_input;
                }
            }
        },
    }
}
