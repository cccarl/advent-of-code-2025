mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::fs::File;
use std::io;
use std::io::BufReader;

use crate::Challenge;
use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;

const INPUTS_FOLDER: &str = "inputs";

pub fn day_picker(chall: Challenge, day_str: String) -> io::Result<()> {
    let input = File::open(format!("{}/day{}.txt", INPUTS_FOLDER, day_str))?;
    let mut reader = BufReader::new(input);

    match chall {
        Challenge::Day1 => day1(reader),
        Challenge::Day2 => day2(reader),
        Challenge::Day3 => day3(reader),
        Challenge::Day4 => day4(reader),
        Challenge::Day5 => day5(reader),
        Challenge::Day6 => day6(&mut reader),
        Challenge::Day7 => day7(reader),
    };

    Ok(())
}
