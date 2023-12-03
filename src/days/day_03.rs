use std::{collections::LinkedList, error::Error, usize};

use super::day::Day;

pub struct Day03;

impl Day for Day03 {
    fn get_day(&self) -> u8 {
        3
    }

    fn part_1(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let symbols = input.iter().enumerate().fold(
            LinkedList::new(),
            |mut acc: LinkedList<(usize, usize)>, (y, line)| {
                acc.append(&mut get_symbols(&line, y));
                acc
            },
        );
        let numbers = input.iter().enumerate().fold(
            LinkedList::new(),
            |mut acc: LinkedList<(usize, (usize, (usize, usize)))>, (y, line)| {
                acc.append(&mut get_numbers(&line, y));
                acc
            },
        );
        Ok(find_adjacent_numbers(numbers, symbols).to_string())
    }

    fn part_2(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let symbols = input.iter().enumerate().fold(
            LinkedList::new(),
            |mut acc: LinkedList<(usize, usize)>, (y, line)| {
                acc.append(&mut get_symbols(&line, y));
                acc
            },
        );
        let numbers = input.iter().enumerate().fold(
            LinkedList::new(),
            |mut acc: LinkedList<(usize, (usize, (usize, usize)))>, (y, line)| {
                acc.append(&mut get_numbers(&line, y));
                acc
            },
        );
        Ok(find_adjacent_symbols(numbers, symbols).to_string())
    }
}

fn get_symbols(line: &str, y: usize) -> LinkedList<(usize, usize)> {
    line.chars()
        .enumerate()
        .fold(LinkedList::new(), |mut acc, (x, c)| {
            if !c.is_numeric() && c.ne(&'.') {
                acc.push_back((x, y))
            }
            acc
        })
}

fn get_numbers(line: &str, y: usize) -> LinkedList<(usize, (usize, (usize, usize)))> {
    line.split(|c: char| !c.is_digit(10))
        .into_iter()
        .fold((0, LinkedList::new()), |(mut prev_index, mut acc), n| {
            if let Ok(number) = n.parse::<usize>() {
                let start_index = line[prev_index..].find(n).unwrap() + prev_index;
                let end_index = start_index + n.len() - 1;
                prev_index = end_index + 1;
                acc.push_back((number, (y, (start_index, end_index))))
            }
            (prev_index, acc)
        })
        .1
}

fn find_adjacent_numbers(
    numbers: LinkedList<(usize, (usize, (usize, usize)))>,
    symbols: LinkedList<(usize, usize)>,
) -> usize {
    numbers
        .iter()
        .fold(0, |mut acc, (number, (y, (start_x, end_x)))| {
            let has_adjacent_symbol = symbols.iter().any(|(s_x, s_y)| {
                ((y.gt(&0) && (y - 1).eq(s_y) || (y + 1).eq(s_y))
                    && ((end_x + 1).eq(s_x)
                        || (start_x.le(s_x) && end_x.ge(s_x))
                        || start_x.gt(&0) && (start_x - 1).eq(s_x)))
                    || (s_y.eq(y)
                        && ((end_x + 1).eq(s_x) || start_x.gt(&0) && (start_x - 1).eq(s_x)))
            });
            if has_adjacent_symbol {
                acc = acc + number
            }
            acc
        })
}

fn find_adjacent_symbols(
    numbers: LinkedList<(usize, (usize, (usize, usize)))>,
    symbols: LinkedList<(usize, usize)>,
) -> usize {
    symbols.iter().fold(0, |mut acc, (s_x, s_y)| {
        let ad_numbers = numbers
            .iter()
            .filter(|(_, (y, (start_x, end_x)))| {
                ((y.gt(&0) && (y - 1).eq(s_y) || (y + 1).eq(s_y))
                    && ((end_x + 1).eq(s_x)
                        || (start_x.le(s_x) && end_x.ge(s_x))
                        || start_x.gt(&0) && (start_x - 1).eq(s_x)))
                    || (s_y.eq(y)
                        && ((end_x + 1).eq(s_x) || start_x.gt(&0) && (start_x - 1).eq(s_x)))
            })
            .map(|(number, _)| number.to_owned())
            .collect::<Vec<usize>>();
        if ad_numbers.len() == 2 {
            acc = acc + ad_numbers.first().unwrap() * ad_numbers.last().unwrap()
        }
        acc
    })
}

#[cfg(test)]
mod test {

    use std::error::Error;

    use crate::days::day::Day;

    use super::Day03;

    static DAY: Day03 = Day03 {};

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "4361";

        assert_eq!(DAY.part_1(&input)?, expected_result);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "467835";

        assert_eq!(DAY.part_2(&input)?, expected_result);

        Ok(())
    }

    fn get_test_input() -> Vec<String> {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        input
            .lines()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }
}
