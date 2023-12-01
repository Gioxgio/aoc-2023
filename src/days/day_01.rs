use std::error::Error;

use super::day::Day;

pub struct Day01;

impl Day for Day01 {
    fn get_day(&self) -> u8 {
        1
    }

    fn part_1(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let result = input
            .iter()
            .map(|v| {
                v.chars()
                    .filter(|c| c.is_digit(10))
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .map(|c| c.first().unwrap() * 10 + c.last().unwrap())
            .sum::<u32>();
        Ok(result.to_string())
    }

    fn part_2(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let result = input
            .iter()
            .map(|v| {
                v.replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "4")
                    .replace("five", "5e")
                    .replace("six", "6")
                    .replace("seven", "7n")
                    .replace("eight", "e8t")
                    .replace("nine", "9e")
            })
            .map(|v| {
                v.chars()
                    .filter(|c| c.is_digit(10))
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .map(|c| c.first().unwrap() * 10 + c.last().unwrap())
            .sum::<u32>();
        Ok(result.to_string())
    }
}
#[cfg(test)]
mod test {

    use std::error::Error;

    use crate::days::day::Day;

    use super::Day01;

    static DAY: Day01 = Day01 {};

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input_1();
        let expected_result = "142";

        assert_eq!(DAY.part_1(&input)?, expected_result);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input_2();
        let expected_result = "281";

        assert_eq!(DAY.part_2(&input)?, expected_result);

        Ok(())
    }

    fn get_test_input_1() -> Vec<String> {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        input
            .lines()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }

    fn get_test_input_2() -> Vec<String> {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        input
            .lines()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }
}
