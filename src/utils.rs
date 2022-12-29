use regex::Regex;
use std::{io, num::ParseIntError};

use crate::models::{City, Uppercase};

pub fn pick_option() -> Result<usize, ParseIntError> {
    println!("Input number: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<usize>()
}

pub fn city_and_country(city: City) -> (String, String) {
    let regex = Regex::new(r"(.+) (\(.+\))").unwrap();
    let captures = &regex.captures(&city).unwrap();
    (
        captures[1].to_owned().to_uppercase_every_word(),
        captures[2].to_uppercase(),
    )
}
