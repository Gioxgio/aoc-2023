use std::error::Error;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::day::Day;

pub struct Day05;

impl Day for Day05 {
    fn get_day(&self) -> u8 {
        5
    }

    fn part_1(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let sections = input.split(|s| s.is_empty()).collect::<Vec<&[String]>>();
        let seeds = sections[0];
        let sections = sections[1..]
            .iter()
            .map(|s| parse_section(s.to_owned()))
            .collect::<Vec<Vec<Vec<i64>>>>();
        let seeds = parse_seeds(seeds.first().unwrap());

        let result = seeds
            .iter()
            .map(|s| process_seed(&sections, 0, *s))
            .min()
            .unwrap();
        Ok(result.to_string())
    }

    fn part_2(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let sections = input.split(|s| s.is_empty()).collect::<Vec<&[String]>>();
        let seeds = sections[0];
        let sections = sections[1..]
            .iter()
            .map(|s| parse_section(s.to_owned()))
            .collect::<Vec<Vec<Vec<i64>>>>();
        let seeds = parse_seeds_part_2(seeds.first().unwrap());

        let result = seeds
            .par_iter()
            .map(|s| process_seed(&sections, 0, *s))
            .min()
            .unwrap();
        Ok(result.to_string())
    }
}

fn parse_seeds_part_2(seeds: &str) -> Vec<i64> {
    seeds
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .as_slice()
        .chunks(2)
        .fold(Vec::new(), |mut acc, r| {
            let mut range = (r[0]..r[0] + r[1]).collect::<Vec<i64>>();
            acc.append(&mut range);
            acc
        })
}

fn parse_seeds(seeds: &str) -> Vec<i64> {
    seeds
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_section(section: &[String]) -> Vec<Vec<i64>> {
    section[1..]
        .iter()
        .map(|s| parse_line(s))
        .collect::<Vec<Vec<i64>>>()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn process_seed(sections: &Vec<Vec<Vec<i64>>>, i: usize, mut value: i64) -> i64 {
    if sections.len().eq(&i) {
        return value;
    }
    let section = sections.get(i).unwrap();
    value = section
        .into_iter()
        .find_map(|l| get_mapped_value(l, value))
        .unwrap_or(value);
    process_seed(sections, i + 1, value)
}

fn get_mapped_value(line: &Vec<i64>, value: i64) -> Option<i64> {
    let dest_start = line.get(0).unwrap();
    let src_start = line.get(1).unwrap();
    let range = line.get(2).unwrap();
    let value = &value;

    if value.ge(src_start) && value.lt(&(src_start + range)) {
        Some(dest_start + (value - src_start))
    } else {
        None
    }
}

#[cfg(test)]
mod test {

    use std::error::Error;

    use crate::days::day::Day;

    use super::Day05;

    static DAY: Day05 = Day05 {};

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "35";

        assert_eq!(DAY.part_1(&input)?, expected_result);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "46";

        assert_eq!(DAY.part_2(&input)?, expected_result);

        Ok(())
    }

    fn get_test_input() -> Vec<String> {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        input
            .lines()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }
}
