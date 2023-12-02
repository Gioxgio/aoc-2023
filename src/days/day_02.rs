use std::error::Error;

use super::day::Day;

pub struct Day02;

impl Day for Day02 {
    fn get_day(&self) -> u8 {
        2
    }

    fn part_1(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let (max_red, max_green, max_blue) = (12, 13, 14);
        let mut valid_games = 0;

        for line in input {
            let line = line.replace("Game ", "");
            if let Some((game_number, sets)) = line.split_once(":") {
                let invalid = sets.split(";").into_iter().map(|s| analyse_set(s)).any(
                    |(red, green, blue)| red > max_red || green > max_green || blue > max_blue,
                );
                if !invalid {
                    valid_games = valid_games + game_number.parse::<i32>().unwrap();
                }
            }
        }

        Ok(valid_games.to_string())
    }

    fn part_2(&self, input: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let mut total_power: i32 = 0;

        for line in input {
            let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
            if let Some((_, sets)) = line.split_once(":") {
                sets.split(";")
                    .into_iter()
                    .map(|s| analyse_set(s))
                    .for_each(|(red, green, blue)| {
                        if red > min_red {
                            min_red = red
                        }
                        if green > min_green {
                            min_green = green
                        }
                        if blue > min_blue {
                            min_blue = blue
                        }
                    });
                total_power = total_power + min_red * min_green * min_blue;
            }
        }

        Ok(total_power.to_string())
    }
}

fn analyse_set(s: &str) -> (i32, i32, i32) {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    let get_value = |s: &str, r: &str| s.replace(r, "").trim().parse::<i32>().unwrap();

    s.split(",").for_each(|c| {
        if c.contains("red") {
            red = get_value(c, "red")
        } else if c.contains("green") {
            green = get_value(c, "green")
        } else if c.contains("blue") {
            blue = get_value(c, "blue")
        }
    });

    (red, green, blue)
}

#[cfg(test)]
mod test {

    use std::error::Error;

    use crate::days::day::Day;

    use super::Day02;

    static DAY: Day02 = Day02 {};

    #[test]
    fn test_part_1() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "8";

        assert_eq!(DAY.part_1(&input)?, expected_result);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = get_test_input();
        let expected_result = "2286";

        assert_eq!(DAY.part_2(&input)?, expected_result);

        Ok(())
    }

    fn get_test_input() -> Vec<String> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        input
            .lines()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }
}
