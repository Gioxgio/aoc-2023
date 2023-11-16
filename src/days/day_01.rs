use std::error::Error;

use super::day::Day;

pub struct Day01;

impl Day for Day01 {
    fn get_day(&self) -> u8 {
        1
    }

    fn part_1(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        Ok(String::from(""))
    }

    fn part_2(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        Ok(String::from(""))
    }
}
