use std::error::Error;

use super::day::Day;

pub struct Day04;

impl Day for Day04 {
    fn get_day(&self) -> u8 {
        4
    }

    fn part_1(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let total_score = input
            .iter()
            .map(|card| parse_input(card))
            .map(|(winning_numbers, current_numbers)| get_score(winning_numbers, current_numbers))
            .sum::<i32>();
        Ok(total_score.to_string())
    }

    fn part_2(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let mut parsed_input = input
            .iter()
            .map(|card| parse_input(card))
            .map(|(winning_numbers, current_numbers)| {
                (
                    get_matches(winning_numbers, current_numbers).len(),
                    1 as i32,
                )
            })
            .collect::<Vec<(usize, i32)>>();

        for i in 0..parsed_input.len() {
            let (matches, count) = parsed_input[i];
            for j in 0..matches.to_owned() {
                parsed_input[j + i + 1].1 += count;
            }
        }

        let cards = parsed_input.iter().map(|(_, count)| count).sum::<i32>();
        Ok(cards.to_string())
    }
}

fn parse_input(card: &str) -> (Vec<i32>, Vec<i32>) {
    let numbers = &card.split([':', '|']).collect::<Vec<&str>>()[1..];
    (
        parse_numbers(numbers.first().unwrap()),
        parse_numbers(numbers.last().unwrap()),
    )
}

fn parse_numbers(numbers: &str) -> Vec<i32> {
    numbers
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn get_matches(winning_numbers: Vec<i32>, current_numbers: Vec<i32>) -> Vec<i32> {
    current_numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .map(|n| n.to_owned())
        .collect::<Vec<i32>>()
}

fn get_score(winning_numbers: Vec<i32>, current_numbers: Vec<i32>) -> i32 {
    let matches = get_matches(winning_numbers, current_numbers);
    if matches.len().ne(&0) {
        return (2 as i32).pow((matches.len() - 1) as u32);
    }
    0
}

#[cfg(test)]
mod test {

    use std::error::Error;

    use crate::days::day::Day;

    use super::Day04;

    static DAY: Day04 = Day04 {};

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "13";

        assert_eq!(DAY.part_1(&input)?, expected_result);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "30";

        assert_eq!(DAY.part_2(&input)?, expected_result);

        Ok(())
    }

    fn get_test_input() -> Vec<String> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        input
            .lines()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }
}
