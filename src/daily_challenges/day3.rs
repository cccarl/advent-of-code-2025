use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// https://adventofcode.com/2025/day/3
pub fn day3(input_reader: BufReader<File>) {
    let mut final_joltage = 0;
    let mut final_joltage_pt2 = 0;
    for line_res in input_reader.lines() {
        let line = match line_res {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error when reading line: {}", e);
                continue;
            }
        };

        let mut index_highest_num: Option<usize> = None;
        let mut index_2nd_highest_num: Option<usize> = None;
        for (i, char) in line.chars().enumerate() {
            let curr_num: i32 = char.to_string().parse().expect("Could not parse number");
            let highest_num: i32 = match index_highest_num {
                Some(index) => line
                    .get(index..index + 1)
                    .expect("Could not get line index (1st)")
                    .parse()
                    .expect("Could not parse to i32"),
                None => -1,
            };

            // last number can never be the big digit
            if curr_num > highest_num && i != line.len() - 1 {
                index_highest_num = Some(i);
                continue;
            }
        }

        for (i, char) in line.chars().enumerate() {
            let curr_num: i32 = char.to_string().parse().expect("Could not parse number");
            let highest_2nd_num: i32 = match index_2nd_highest_num {
                Some(index) => line
                    .get(index..index + 1)
                    .expect("Could not get line index (2nd)")
                    .parse()
                    .expect("Could not parse to i32"),
                None => -1,
            };

            if curr_num > highest_2nd_num
                && index_highest_num.is_some()
                && i > index_highest_num.unwrap()
            {
                index_2nd_highest_num = Some(i);
            }
        }

        let jolt_dec = line
            .get(index_highest_num.unwrap()..index_highest_num.unwrap() + 1)
            .expect("Could not get line index (1st)")
            .parse::<i32>()
            .expect("Could not parse to i32");
        let jolt_unit = line
            .get(index_2nd_highest_num.unwrap()..index_2nd_highest_num.unwrap() + 1)
            .expect("Could not get line index (2nd)")
            .parse::<i32>()
            .expect("Could not parse to i32");

        let joltage = jolt_dec * 10 + jolt_unit;

        final_joltage += joltage;

        //println!("{}, jotage -> {}", line, joltage);

        // part 2: the same but with 12 chars so let's not hardcode the 2 loops
        let mut indexes_with_highest_nums: Vec<Option<usize>> = vec![
            None, None, None, None, None, None, None, None, None, None, None, None,
        ];
        let total_loops = indexes_with_highest_nums.len();
        let mut highest_index: i32 = -1;
        for (i, index_in_vec) in indexes_with_highest_nums
            .iter_mut()
            .enumerate()
            .take(total_loops)
        {
            //println!("Digit {} is checking: {}", i, line.get(0..(line.len() - (total_loops - i - 1))).unwrap());

            for (index, char) in line.chars().enumerate() {
                let curr_num: i32 = char.to_string().parse().expect("Could not parse number");
                let highest_num: i32 = match index_in_vec {
                    Some(index) => line
                        .get(*index..*index + 1)
                        .expect("Could not get line index")
                        .parse()
                        .expect("Could not parse to i32"),
                    None => -1,
                };

                if curr_num > highest_num
                    && index < line.len() - (total_loops - i - 1)
                    && (index as i32) > highest_index
                {
                    *index_in_vec = Some(index);
                    highest_index = index.try_into().unwrap();
                    //println!("[Digit {}]Compared: {} vs {}, saving: {} at index {}", i, curr_num, highest_num, curr_num, index );
                    continue;
                }
            }
        }

        let mut final_jolt_line = 0;
        indexes_with_highest_nums.reverse(); // make the lowest value first
        for (index, i_opt) in indexes_with_highest_nums.iter().enumerate() {
            let crash_here_lawl = i_opt.unwrap();
            let jolt = line
                .get(crash_here_lawl..crash_here_lawl + 1)
                .expect("Could not get line index")
                .parse::<u64>()
                .expect("Could not parse to i32");
            //println!("tha jolt in index {} is: {}", index, jolt);
            final_jolt_line += jolt * 10_u64.pow(index.try_into().unwrap());
        }
        println!("{}, jotage -> {}", line, final_jolt_line);
        final_joltage_pt2 += final_jolt_line;
    }

    println!("[part 1] Final JOLTAGE: {}", final_joltage);
    println!("[part 2] Final JOLTAGE: {}", final_joltage_pt2);
}
