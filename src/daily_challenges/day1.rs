use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// https://adventofcode.com/2025/day/1
pub fn day1(input_reader: BufReader<File>) {
    // part 1: get true password
    let mut current_score = 50;
    let mut final_password = 0;
    let mut final_password_pt2 = 0;
    for line_res in input_reader.lines() {
        let line = match line_res {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error when reading line: {}", e);
                continue;
            }
        };

        if let Some(score_str) = line.get(1..) {
            let score_in_line: Result<i32, _> = score_str.parse();

            match score_in_line {
                Ok(score) => {
                    if line.contains("R") {
                        current_score += score;
                        while current_score > 99 {
                            current_score -= 100;
                            final_password_pt2 += 1;
                        }
                    } else if line.contains("L") {
                        let mut started_in_zero = false;
                        if current_score == 0 {
                            started_in_zero = true;
                        }
                        current_score -= score;
                        while current_score < 0 {
                            current_score += 100;
                            final_password_pt2 += 1;
                        }
                        if current_score == 0 {
                            final_password_pt2 += 1;
                        }
                        if started_in_zero {
                            final_password_pt2 -= 1;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("error n shit in the number parse: {}", e);
                }
            }
            //println!("<{}> Current score: {}", line, current_score);
            println!(
                "<{}> Current score and pass (pt2): {} - {}",
                line, current_score, final_password_pt2
            );
            // part 1 solution
            if current_score == 0 {
                final_password += 1;
            }
        }
    }

    println!("Part 1: The final password is: {}", final_password);
    println!(
        "Part 2: The final with any click password is: {}",
        final_password_pt2
    );
}
