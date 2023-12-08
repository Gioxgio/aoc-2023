use std::{collections::HashMap, error::Error};

use num::Integer;

use super::day::Day;

pub struct Day08;

impl Day for Day08 {
    fn get_day(&self) -> u8 {
        8
    }

    fn part_1(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let (instructions, input) = parse_input(input);
        let key = String::from("AAA");
        let end = Box::new(|k: &str| k.eq("ZZZ"));
        let count = get_steps(instructions, &input, key, end);
        Ok(count.to_string())
    }

    fn part_2(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let (instructions, input) = parse_input(input);
        let keys = get_keys(&input);
        let end = Box::new(|k: &str| k.ends_with('Z'));

        let result = keys
            .iter()
            .map(|key| get_steps(instructions.clone(), &input, String::from(key), end.clone()))
            .fold(1 as i64, |acc, n| acc.lcm(&(n as i64)));
        Ok(result.to_string())
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<char>, HashMap<String, (String, String)>) {
    let instructions = input
        .first()
        .unwrap()
        .to_owned()
        .chars()
        .collect::<Vec<char>>();
    let input = input[2..].iter().fold(
        HashMap::new(),
        |mut acc: HashMap<String, (String, String)>, l| {
            let (key, value) = parse_line(l);
            acc.insert(key, value);
            acc
        },
    );

    (instructions, input)
}

fn parse_line(line: &str) -> (String, (String, String)) {
    let line = line.replace(&['=', '(', ',', ')'], "");
    let segments = line
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    (
        segments.get(0).unwrap().to_owned(),
        (
            segments.get(1).unwrap().to_owned(),
            segments.get(2).unwrap().to_owned(),
        ),
    )
}

fn get_steps(
    instructions: Vec<char>,
    input: &HashMap<String, (String, String)>,
    mut key: String,
    end: Box<dyn Fn(&str) -> bool>,
) -> i32 {
    let mut index = 0;
    let mut count = 0;
    while !end(&key) {
        if index.eq(&instructions.len()) {
            index = 0
        }
        let values = input.get(&key).unwrap();
        key = match instructions.get(index).unwrap() {
            'L' => &values.0,
            'R' => &values.1,
            _ => unreachable!(),
        }
        .to_owned();
        index = index + 1;
        count = count + 1;
    }
    count
}

fn get_keys(input: &HashMap<String, (String, String)>) -> Vec<String> {
    input
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod test {

    use std::error::Error;

    use crate::days::day::Day;

    use super::Day08;

    static DAY: Day08 = Day08 {};

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input_1();
        let expected_result = "6";

        assert_eq!(DAY.part_1(&input)?, expected_result);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input_2();
        let expected_result = "6";

        assert_eq!(DAY.part_2(&input)?, expected_result);

        Ok(())
    }

    fn get_test_input_1() -> Vec<String> {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        input
            .lines()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }

    fn get_test_input_2() -> Vec<String> {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        input
            .lines()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }
}
