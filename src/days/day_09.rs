use std::error::Error;

use super::day::Day;

pub struct Day09;

impl Day for Day09 {
    fn get_day(&self) -> u8 {
        9
    }

    fn part_1(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let result = input
            .iter()
            .map(|line| parse_line(line))
            .map(|line| process_line(line))
            .sum::<i32>();

        Ok(result.to_string())
    }

    fn part_2(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let result = input
            .iter()
            .map(|line| parse_line(line))
            .map(|line| process_line_2(line))
            .sum::<i32>();

        Ok(result.to_string())
    }
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn process_line(mut line: Vec<i32>) -> i32 {
    let mut result = line.last().unwrap().to_owned();
    while !all_the_same(&line) {
        line = get_next_line(line);
        result = result + line.last().unwrap();
    }
    result
}

fn process_line_2(mut line: Vec<i32>) -> i32 {
    let mut toggle = 1;
    let mut result = line.first().unwrap().to_owned();
    while !all_the_same(&line) {
        line = get_next_line(line);
        result = result - line.first().unwrap() * toggle;
        toggle = toggle * -1;
    }
    result
}

fn get_next_line(line: Vec<i32>) -> Vec<i32> {
    line.windows(2).fold(Vec::new(), |mut acc: Vec<i32>, v| {
        acc.push(v[1] - v[0]);
        acc
    })
}

fn all_the_same(line: &Vec<i32>) -> bool {
    line.iter().all(|n| n.eq(&line[0]))
}

#[cfg(test)]
mod test {

    use std::error::Error;

    use crate::days::day::Day;

    use super::Day09;

    static DAY: Day09 = Day09 {};

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "114";

        assert_eq!(DAY.part_1(&input)?, expected_result);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "2";

        assert_eq!(DAY.part_2(&input)?, expected_result);

        Ok(())
    }

    fn get_test_input() -> Vec<String> {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        input
            .lines()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }
}
