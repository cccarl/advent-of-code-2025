mod day1;
mod day2;
mod day3;

use std::fs::File;
use std::io;
use std::io::BufReader;

use crate::Challenge;
use day1::day1;
use day2::day2;
use day3::day3;

const INPUTS_FOLDER: &str = "inputs";

pub fn day_picker(chall: Challenge, day_str: String) -> io::Result<()> {
    let input = File::open(format!("{}/day{}.txt", INPUTS_FOLDER, day_str))?;
    let reader = BufReader::new(input);

    match chall {
        Challenge::Day1 => day1(reader),
        Challenge::Day2 => day2(reader),
        Challenge::Day3 => day3(reader),
    };

    Ok(())
}
