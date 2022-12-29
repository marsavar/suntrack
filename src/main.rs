use asciifolding::fold_to_ascii;
use chrono::{Local, TimeZone, Utc};
use clap::Parser;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, io, num::ParseIntError, process};

use crate::cli::Cli;

mod cli;

type City = String;

#[derive(Debug, Deserialize)]
struct Row {
    city: String,
    lat: f64,
    long: f64,
    iso: String,
}

#[derive(Debug)]
struct Coords {
    lat: f64,
    long: f64,
}

trait Uppercase {
    fn to_uppercase_every_word(&self) -> String;
}

impl Uppercase for String {
    fn to_uppercase_every_word(&self) -> String {
        self.split_ascii_whitespace()
            .map(|a| a[0..1].to_uppercase() + &a[1..])
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn pick_option() -> Result<usize, ParseIntError> {
    println!("Input number: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<usize>()
}

struct MapOfCities {
    map: HashMap<City, Coords>,
}

impl MapOfCities {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn print_coords(&self, cities: Vec<&City>, index: usize, local: bool) {
        let Coords { lat, long } = self.map[cities[index]];

        let city = cities[index].to_uppercase_every_word();

        let (sunrise, sunset) = sunrise::sunrise_sunset(lat, long, 2022, 12, 29);
        let (sunrise, sunset) = if local {
            (
                Local.timestamp_opt(sunrise, 0).unwrap().to_string(),
                Local.timestamp_opt(sunset, 0).unwrap().to_string(),
            )
        } else {
            (
                Utc.timestamp_opt(sunrise, 0).unwrap().to_string(),
                Utc.timestamp_opt(sunset, 0).unwrap().to_string(),
            )
        };

        let regex = Regex::new(r"(.+) (\(.+\))").unwrap();
        let captures = &regex.captures(&city).unwrap();

        let (city, country) = (&captures[1], &captures[2].to_uppercase());

        println!("🌍 {} {}", city, country);
        println!("🌄 Sunrise: {sunrise}");
        println!("🌆 Sunset: {sunset}");
    }
}
fn main() {
    let mut all_cities = MapOfCities::new();

    let cities = include_str!("cities.csv");
    let mut rdr = csv::ReaderBuilder::new().from_reader(cities.as_bytes());

    for result in rdr.deserialize() {
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
