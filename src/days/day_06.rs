use std::error::Error;

use super::day::Day;

pub struct Day06;

impl Day for Day06 {
    fn get_day(&self) -> u8 {
        6
    }

    fn part_1(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let times = parse_line(input.get(0).unwrap());
        let records = parse_line(input.get(1).unwrap());
        let mut result = 1;

        for i in 0..times.len() {
            if let Some(n) = find_times(times.get(i).unwrap(), records.get(i).unwrap()) {
                result = result * n
            }
        }
        Ok(result.to_string())
    }

    fn part_2(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let time = parse_line_2(input.get(0).unwrap());
        let record = parse_line_2(input.get(1).unwrap());
        let result = find_times(&time, &record).unwrap();

        Ok(result.to_string())
    }
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_line_2(line: &str) -> i64 {
    line.split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}

fn find_times(time: &i64, record: &i64) -> Option<i64> {
    let half = (*time as f32 / 2 as f32).floor() as i64;
    for i in 1..half {
        if (time - i) * i > *record {
            return Some(time - (i * 2 - 1));
        }
    }
    None
}

#[cfg(test)]
mod test {

    use std::error::Error;

    use crate::days::day::Day;

    use super::Day06;

    static DAY: Day06 = Day06 {};

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "288";

        assert_eq!(DAY.part_1(&input)?, expected_result);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "71503";

        assert_eq!(DAY.part_2(&input)?, expected_result);

        Ok(())
    }

    fn get_test_input() -> Vec<String> {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        input
            .lines()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }
}
