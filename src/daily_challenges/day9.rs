use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// https://adventofcode.com/2025/day/9
pub fn day9(input_reader: BufReader<File>) {

    let mut coords: Vec<(i64, i64)> = Vec::new();
    for line_res in input_reader.lines() {
        let line = match line_res {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error when reading line: {}", e);
                continue;
            }
        };

        let mut line_split = line.split(',');
        coords.push((
            line_split.next().expect("WRONG char 1").parse().expect("Could not make a u32"),
            line_split.next().expect("WRONG char 2").parse().expect("Could not make a u32"),
        ));
    }
    println!("coords: {:?}", coords);

    let mut biggest_area = 0;
    for (i, i_coord) in coords.iter().enumerate() {
        for (j, j_coord) in coords.iter().enumerate() {
            // avoid comparing the same space and repeats
            if i >= j {
                continue;
            }

            let curr_area = (i_coord.0 - j_coord.0 + 1) * (i_coord.1 - j_coord.1 + 1);
            if curr_area > biggest_area {
                biggest_area = curr_area;
            }
        }
    }
    println!("[Part 1] Biggest area: {}", biggest_area);
}
