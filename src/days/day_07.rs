use std::{cmp::Ordering, collections::HashMap, error::Error};

use super::day::Day;

pub struct Day07;

impl Day for Day07 {
    fn get_day(&self) -> u8 {
        7
    }

    fn part_1(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let mut input = input
            .iter()
            .map(|l| parse_line(l))
            .collect::<Vec<(Vec<char>, i32)>>();
        input.sort_by(|(a, _), (b, _)| sorter(a, b));
        let result = input
            .iter()
            .enumerate()
            .fold(0, |acc, (i, (_, n))| acc + (i as i32 + 1) * n);
        Ok(result.to_string())
    }

    fn part_2(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let mut input = input
            .iter()
            .map(|l| parse_line(l))
            .collect::<Vec<(Vec<char>, i32)>>();
        input.sort_by(|(a, _), (b, _)| sorter_2(a, b));
        let result = input
            .iter()
            .enumerate()
            .fold(0, |acc, (i, (_, n))| acc + (i as i32 + 1) * n);
        Ok(result.to_string())
    }
}

fn parse_line(line: &str) -> (Vec<char>, i32) {
    let line = line.split_whitespace().collect::<Vec<&str>>();
    let hand = line.first().unwrap().chars().collect::<Vec<char>>();
    let score = line.last().unwrap().parse::<i32>().unwrap();
    (hand, score)
}

fn get_strenght(hand: &Vec<char>) -> i32 {
    let folded_hand = hand
        .iter()
        .fold(HashMap::new(), |mut acc: HashMap<char, i32>, c| {
            let count = acc.entry(*c).or_insert(0);
            *count += 1;
            acc
        });
    match folded_hand.len() {
        1 => 1,
        2 => {
            if folded_hand.values().any(|v| v.eq(&4)) {
                return 2;
            } else {
                return 3;
            }
        }
        3 => {
            if folded_hand.values().any(|v| v.eq(&3)) {
                return 4;
            } else {
                return 5;
            }
        }
        4 => 6,
        5 => 7,
        _ => unreachable!(),
    }
}

fn get_strenght_2(hand: &Vec<char>) -> i32 {
    let mut folded_hand = hand
        .iter()
        .fold(HashMap::new(), |mut acc: HashMap<char, i32>, c| {
            let count = acc.entry(*c).or_insert(0);
            *count += 1;
            acc
        });
    balance_jokers(&mut folded_hand);
    match folded_hand.len() {
        1 => 1,
        2 => {
            if folded_hand.values().any(|v| v.eq(&4)) {
                return 2;
            } else {
                return 3;
            }
        }
        3 => {
            if folded_hand.values().any(|v| v.eq(&3)) {
                return 4;
            } else {
                return 5;
            }
        }
        4 => 6,
        5 => 7,
        _ => unreachable!(),
    }
}

fn balance_jokers(hand: &mut HashMap<char, i32>) {
    if let Some(n) = hand.clone().get(&'J') {
        if n.eq(&5) {
            hand.insert('A', 1);
        }
        hand.remove(&'J');
        let value = hand.values_mut().max().unwrap();
        *value += n;
    }
}

fn compare_cards(hand_1: &Vec<char>, hand_2: &Vec<char>) -> Ordering {
    let order = "AKQJT98765432";
    for i in 0..hand_1.len() {
        if hand_1[i].ne(&hand_2[i]) {
            return order
                .find(hand_1[i])
                .unwrap()
                .cmp(&order.find(hand_2[i]).unwrap());
        }
    }
    unreachable!()
}

fn compare_cards_2(hand_1: &Vec<char>, hand_2: &Vec<char>) -> Ordering {
    let order = "AKQT98765432J";
    for i in 0..hand_1.len() {
        if hand_1[i].ne(&hand_2[i]) {
            return order
                .find(hand_1[i])
                .unwrap()
                .cmp(&order.find(hand_2[i]).unwrap());
        }
    }
    unreachable!()
}

fn sorter(hand_1: &Vec<char>, hand_2: &Vec<char>) -> Ordering {
    let strenght_1 = get_strenght(hand_1);
    let strenght_2 = get_strenght(hand_2);

    if strenght_1.eq(&strenght_2) {
        return compare_cards(hand_2, hand_1);
    }
    strenght_2.cmp(&strenght_1)
}

fn sorter_2(hand_1: &Vec<char>, hand_2: &Vec<char>) -> Ordering {
    let strenght_1 = get_strenght_2(hand_1);
    let strenght_2 = get_strenght_2(hand_2);

    if strenght_1.eq(&strenght_2) {
        return compare_cards_2(hand_2, hand_1);
    }
    strenght_2.cmp(&strenght_1)
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
        let expected_result = "6440";

        assert_eq!(DAY.part_1(&input)?, expected_result);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "5905";

        assert_eq!(DAY.part_2(&input)?, expected_result);

        Ok(())
    }

    fn get_test_input() -> Vec<String> {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        input
            .lines()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }
}
