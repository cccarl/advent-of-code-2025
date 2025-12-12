use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

#[derive(Debug)]
enum OpType {
    Add,
    Mul,
}

// https://adventofcode.com/2025/day/6
pub fn day6(input_reader: &mut BufReader<File>) {
    let mut operation_data: Vec<Vec<u64>> = vec![];
    let mut operation_types: Vec<OpType> = vec![];
    for line_res in input_reader.lines() {
        let line = match line_res {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error when reading line: {}", e);
                continue;
            }
        };

        let mut current_line_values: Vec<u64> = vec![];
        let line_split = line.split(' ');
        for pot_number in line_split {
            if pot_number.is_empty() {
                continue;
            }

            let number_parsed = pot_number.parse::<u64>();
            match number_parsed {
                Ok(num) => {
                    current_line_values.push(num);
                }
                // probably a letter then
                Err(_) => {
                    if pot_number == "+" {
                        operation_types.push(OpType::Add);
                    } else if pot_number == "*" {
                        operation_types.push(OpType::Mul);
                    }
                }
            }
        }
        if !current_line_values.is_empty() {
            operation_data.push(current_line_values);
        }
    }

    let mut final_sum = 0;
    for i in 0..operation_data[0].len() {
        let mut curr_op_nums: Vec<u64> = vec![];
        let operation = &operation_types[i];
        for j in 0..operation_data.len() {
            curr_op_nums.push(operation_data[j][i]);
        }
        let result = match operation {
            OpType::Add => curr_op_nums.iter().sum::<u64>(),
            OpType::Mul => curr_op_nums.iter().product::<u64>(),
        };

        final_sum += result;
    }

    println!("[Part 1] the final sum is: {}", final_sum);

    // part 2: actually parsing the numbers was completely different, oops!
    let rew_res = input_reader.rewind();
    if rew_res.is_err() {
        panic!("IDK BRO rewind just didn't work");
    }

    let mut number_lines_chars: Vec<Vec<char>> = vec![];
    let mut col_indexes_num_starts: Vec<usize> = vec![];
    let mut last_row_mode = false;
    for line_res in input_reader.lines() {
        let line = match line_res {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error when reading line: {}", e);
                continue;
            }
        };

        let mut current_line_chars = vec![];
        for (idx, char) in line.chars().enumerate() {
            if !last_row_mode && (char == '*' || char == '+') {
                last_row_mode = true;
            }

            if last_row_mode {
                // these consistently show where the number columns start
                if char == '*' || char == '+' {
                    col_indexes_num_starts.push(idx);
                }
            } else {
                current_line_chars.push(char);
            }
        }
        if !current_line_chars.is_empty() {
            number_lines_chars.push(current_line_chars);
        }
    }

    // parse the chars into the actual numbers: make numbers from columns
    let mut all_operation_nums: Vec<Vec<u64>> = vec![];
    for numbers_start_idx in col_indexes_num_starts {
        let mut curr_op_nums = vec![];
        let mut valid_column = true;
        let mut offset = 0;
        while valid_column {
            let mut potential_number_built = String::new();
            for line in &number_lines_chars {
                let char = line.get(numbers_start_idx + offset);

                if let Some(c) = char {
                    if *c != ' ' {
                        potential_number_built.push(*c);
                    }
                }
            }

            let number_parsed = potential_number_built.parse::<u64>();
            match number_parsed {
                Ok(num) => curr_op_nums.push(num),
                Err(_) => valid_column = false,
            }
            offset += 1;
        }
        //println!("Numbers in this col: {:?}", curr_op_nums);
        all_operation_nums.push(curr_op_nums);
    }

    let mut final_sum_pt2 = 0;
    // assume same length
    for (numbers, operation) in all_operation_nums.iter().zip(operation_types.iter()) {
        let mut curr_result = 0;
        println!("Operation: {:?} {:?}", numbers, operation);
        match operation {
            OpType::Add => curr_result += numbers.iter().sum::<u64>(),
            OpType::Mul => curr_result += numbers.iter().product::<u64>(),
        }
        final_sum_pt2 += curr_result;
    }

    println!(
        "[Part 2] Final sum with the stupid column format: {}",
        final_sum_pt2
    );
}
