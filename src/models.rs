use std::collections::HashMap;

use chrono::{Local, TimeZone, Utc};
use regex::Regex;
use serde::Deserialize;

pub type City = String;

#[derive(Debug, Deserialize)]
pub struct Row {
    pub city: String,
    pub lat: f64,
    pub long: f64,
    pub iso: String,
}

#[derive(Debug)]
pub struct Coords {
    pub lat: f64,
    pub long: f64,
}

pub struct MapOfCities {
    pub map: HashMap<City, Coords>,
}

impl MapOfCities {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn print_coords(&self, cities: Vec<&City>, index: usize, local: bool) {
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

pub trait Uppercase {
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
