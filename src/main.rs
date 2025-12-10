use std::env;

use crate::daily_challenges::day_picker;

mod daily_challenges;

pub enum Challenge {
    Day1,
}

fn main() {
    // save the first command (the exercise to execute)
    let ex = env::args().nth(1);

    let ex_result = match ex {
        Some(day) => match day.as_str() {
            "1" => day_picker(Challenge::Day1, day),
            _ => {
                println!("Invalid day!!!: {}", day);
                Ok(())
            },
        },
        None => {
            println!("Please enter the challenge day number in console.");
            Ok(())
        }
    };

    if let Err(read_error) = ex_result {
        eprintln!("Error in day picker: {}", read_error);
    };
}
