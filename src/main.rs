use std::error::Error;

use aoc_2022::{
    days::{day::Day, day_01::Day01, day_02::Day02},
    read_lines,
};

fn main() -> Result<(), Box<dyn Error>> {
    let days: Vec<&dyn Day> = vec![&Day01 {}, &Day02 {}];

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
