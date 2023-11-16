use std::error::Error;

pub trait Day {
    fn get_day(&self) -> u8;
    fn part_1(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>>;
    fn part_2(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>>;
}
