use std::error::Error;

use aoc_2023::{
    days::{
        day::Day, day_01::Day01, day_02::Day02, day_03::Day03, day_04::Day04, day_05::Day05,
        day_06::Day06, day_07::Day07,
    },
    read_lines,
};

fn main() -> Result<(), Box<dyn Error>> {
    let days: Vec<&dyn Day> = vec![&Day01, &Day02, &Day03, &Day04, &Day05, &Day06, &Day07];

    for day in days {
        let number = day.get_day();

        let input = read_lines(number)?;

        let part_1 = day.part_1(&input)?;
        let part_2 = day.part_2(&input)?;

        println!(
            "Day {:0>2} - Part 1: {} - Part 2: {}",
            number, part_1, part_2
        );
    }

    Ok(())
}
