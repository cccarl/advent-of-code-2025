mod day1;

use std::fs::File;
use std::io;
use std::io::BufReader;

use crate::Challenge;
use crate::daily_challenges::day1::day1;

const INPUTS_FOLDER: &str = "inputs";

pub fn day_picker(chall: Challenge, day_str: String) -> io::Result<()> {
    let input = File::open(format!("{}/day{}.txt", INPUTS_FOLDER, day_str))?;
    let reader = BufReader::new(input);

    match chall {
        Challenge::Day1 => day1(reader),
    };

    Ok(())
}
