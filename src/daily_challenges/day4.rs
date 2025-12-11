use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// https://adventofcode.com/2025/day/3
pub fn day4(input_reader: BufReader<File>) {
    // true -> there's paper in that spot
    let mut paper_field: Vec<Vec<bool>> = vec![];
    // parse input
    for line_res in input_reader.lines() {
        let line = match line_res {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error when reading line: {}", e);
                continue;
            }
        };

        let mut field_row = vec![];
        for char in line.chars() {
            if char == '@' {
                field_row.push(true);
            } else {
                field_row.push(false);
            }
        }
        paper_field.push(field_row);
    }

    let mut paper_free_for_forklift = 0;
    for (y_index, row) in paper_field.iter().enumerate() {
        for (x_index, spot) in row.iter().enumerate() {
            if !spot {
                continue;
            }

            let mut paper_amount_in_proximity = 0;
            for i in -1_i32..2 {
                for j in -1_i32..2 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    let current_index_row = y_index as i32 + i;
                    let current_index_spot = x_index as i32 + j;
                    if current_index_row < 0 || current_index_spot < 0 {
                        continue;
                    }
                    let prox_row_checking =
                        paper_field.get::<usize>(current_index_row.try_into().unwrap());
                    if let Some(field_row_found) = prox_row_checking {
                        let spot_checking =
                            field_row_found.get::<usize>(current_index_spot.try_into().unwrap());
                        if let Some(spot_found) = spot_checking {
                            if *spot_found {
                                paper_amount_in_proximity += 1;
                            }
                        }
                    }
                }
            }

            if paper_amount_in_proximity < 4 {
                paper_free_for_forklift += 1;
            }
            println!(
                "[{}-{}] has {} paper in proximity ({} valid total)",
                x_index, y_index, paper_amount_in_proximity, paper_free_for_forklift
            );
        }
    }

    println!(
        "[Part 1] Spots with paper free for forklift: {}",
        paper_free_for_forklift
    );
}
