use std::{error::Error, fs};

pub mod days;

pub fn read_lines(day: u8) -> Result<Vec<String>, Box<dyn Error>> {
    let file_name = format!("resources/inputs/day_{:0>2}.txt", day);

    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(String::from)
        .collect())
}
