use std::collections::HashMap;

use chrono::{Datelike, Local, NaiveDate, TimeZone, Utc};
use serde::Deserialize;

use crate::utils::city_and_country;

pub type City = String;

#[derive(Debug, Deserialize)]
pub struct Row {
    pub city: String,
    pub lat: f64,
    pub long: f64,
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

    pub fn print_times(
        &self,
        cities: Vec<&City>,
        index: usize,
        local: bool,
        date: Option<NaiveDate>,
    ) {
        let Coords { lat, long } = self.map[cities[index]];

        let date = date.unwrap_or_else(|| {
            if local {
                let today = Utc::now();
                NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap()
            } else {
                let today = Local::now();
                NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap()
            }
        });

        let (sunrise, sunset) =
            sunrise::sunrise_sunset(lat, long, date.year(), date.month(), date.day());

        let (city, country) = city_and_country(cities[index].to_uppercase_every_word());

        let (sunrise, sunset) = if sunrise == 0 && sunset == 0 {
            ("No sunrise".to_owned(), "No sunset".to_owned())
        } else {
            match local {
                true => (
                    // Local system time
                    Local.timestamp_opt(sunrise, 0).unwrap().to_string(),
                    Local.timestamp_opt(sunset, 0).unwrap().to_string(),
                ),
                false => (
                    // UTC
                    Utc.timestamp_opt(sunrise, 0).unwrap().to_string(),
                    Utc.timestamp_opt(sunset, 0).unwrap().to_string(),
                ),
            }
        };

        println!("🌍 City \t{city} {country}\n🌄 Sunrise\t{sunrise}\n🌆 Sunset\t{sunset}",);
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
