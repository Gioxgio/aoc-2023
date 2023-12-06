use std::error::Error;

use super::day::Day;

pub struct Day07;

impl Day for Day07 {
    fn get_day(&self) -> u8 {
        7
    }

    fn part_1(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        Ok(String::from(""))
    }

    fn part_2(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        Ok(String::from(""))
    }
}

#[cfg(test)]
mod test {

    use std::error::Error;

    use crate::days::day::Day;

    use super::Day07;

    static DAY: Day07 = Day07 {};

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "";

        assert_eq!(DAY.part_1(&input)?, expected_result);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "";

        assert_eq!(DAY.part_2(&input)?, expected_result);

        Ok(())
    }

    fn get_test_input() -> Vec<String> {
        let input = "";

        input
            .lines()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }
}
