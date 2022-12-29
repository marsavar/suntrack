use std::{io, num::ParseIntError};

pub fn pick_option() -> Result<usize, ParseIntError> {
    println!("Input number: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<usize>()
}
